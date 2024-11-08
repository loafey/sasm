use monostate::MustBe;
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc};

use crate::{
    parser::{ExportDesc, TypeIdX},
    runtime::{Frame, Runtime, RuntimeError, Value},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ConstValue {
    I32 { value: String },
    I64 { value: String },
    F32 { value: String },
    F64 { value: String },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
enum Action {
    #[serde(rename = "invoke")]
    Invoke {
        module: Option<String>,
        field: String,
        args: Vec<ConstValue>,
    },
    #[serde(rename = "get")]
    Get {
        module: Option<String>,
        field: String,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum ModuleType {
    Binary,
    Text,
}

#[derive(Debug, Deserialize, Clone)]
struct Module {
    #[serde(rename = "type")]
    _type: MustBe!("module"),
    name: Option<String>,
    filename: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertReturn {
    #[serde(rename = "type")]
    _type: MustBe!("assert_return"),
    action: Action,
    expected: Vec<ConstValue>,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertExhaustion {
    #[serde(rename = "type")]
    _type: MustBe!("assert_return"),
    action: Action,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertTrap {
    #[serde(rename = "type")]
    _type: MustBe!("assert_trap"),
    action: Action,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertInvalid {
    #[serde(rename = "type")]
    _type: MustBe!("assert_invalid"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertMalformed {
    #[serde(rename = "type")]
    _type: MustBe!("assert_malformed"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertUninstantiable {
    #[serde(rename = "type")]
    _type: MustBe!("assert_uninstantiable"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
struct AssertUnlinkable {
    #[serde(rename = "type")]
    _type: MustBe!("assert_unlinkable"),
    filename: PathBuf,
    text: String,
    module_type: ModuleType,
}

#[derive(Debug, Deserialize, Clone)]
struct Register {
    #[serde(rename = "type")]
    _type: MustBe!("assert_register"),
    name: String,
    #[serde(rename = "as")]
    _as: String,
}

#[derive(Debug, Deserialize, Clone)]
struct ActionWrap {
    #[serde(rename = "type")]
    _type: MustBe!("action"),
    action: Action,
}

/// https://github.com/WebAssembly/wabt/blob/main/docs/wast2json.md#json-format
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum Case {
    Module(Module),
    AssertReturn(AssertReturn),
    Action(ActionWrap),
    AssertExhaustion(AssertExhaustion),
    AssertTrap(AssertTrap),
    AssertInvalid(AssertInvalid),
    AssertMalformed(AssertMalformed),
    AssertUninstantiable(AssertUninstantiable),
    AssertUnlinkable(AssertUnlinkable),
    Register(Register),
}

#[derive(Debug, Deserialize)]
struct TestCases {
    commands: Vec<Case>,
}

fn const_to_val(consts: Vec<ConstValue>) -> Vec<Value> {
    consts
        .into_iter()
        .map(|v| match v {
            ConstValue::I32 { value } => Value::I32(
                value
                    .parse()
                    .or_else(|_| {
                        value
                            .parse()
                            .map(|v| unsafe { std::mem::transmute::<u32, i32>(v) })
                    })
                    .expect("failed to parse"),
            ),
            ConstValue::I64 { value } => Value::I64(
                value
                    .parse()
                    .or_else(|_| {
                        value
                            .parse()
                            .map(|v| unsafe { std::mem::transmute::<u64, i64>(v) })
                    })
                    .expect("failed to parse"),
            ),
            ConstValue::F32 { value } => Value::F32(unsafe {
                f32::from_bits(
                    value
                        .parse::<i32>()
                        .or_else(|_| value.parse().map(|v| std::mem::transmute::<u32, i32>(v)))
                        .expect("failed to parse") as u32,
                )
            }),
            ConstValue::F64 { value } => Value::F64(unsafe {
                f64::from_bits(
                    value
                        .parse::<i64>()
                        .or_else(|_| value.parse().map(|v| std::mem::transmute::<u64, i64>(v)))
                        .expect("failed to parse") as u64,
                )
            }),
        })
        .collect()
}

fn remove_floats(vals: Vec<Value>) -> Vec<Value> {
    vals.into_iter()
        .map(|v| match v {
            Value::F32(x) => Value::I32(unsafe { std::mem::transmute::<u32, i32>(x.to_bits()) }),
            Value::F64(x) => Value::I64(unsafe { std::mem::transmute::<u64, i64>(x.to_bits()) }),
            x => x,
        })
        .collect()
}

fn handle_action<T>(
    rt: &mut Runtime,
    action: Action,
    loop_func: impl FnOnce(&mut Runtime, &String) -> T,
) -> T {
    match action {
        Action::Invoke {
            module,
            field,
            args,
        } => {
            if module.is_some() {
                todo!()
            }

            let fid = rt.exports.get(&field).expect("no function");

            let ExportDesc::Func(TypeIdX(fid)) = fid else {
                panic!("no function with this id")
            };

            let args = const_to_val(args)
                .into_iter()
                .enumerate()
                .map(|(a, b)| (a as u32, b))
                .collect::<HashMap<_, _>>();

            rt.stack.push(Frame {
                func_id: *fid,
                pc: 0,
                stack: Vec::new(),
                locals: args,
                depth_stack: Vec::new(),
            });

            loop_func(rt, &field)
        }
        Action::Get { module, field } => todo!(),
    }
}

pub fn test(mut path: String) {
    let input = path.to_string();
    path = path.replace(".wast", ".wasm");
    if !PathBuf::from(&path).exists() {
        std::process::exit(0);
    }
    std::process::Command::new("wast2json")
        .arg(input)
        .arg("-o")
        .arg(&path)
        .output()
        .expect("Failed to run wast2json");

    let p = PathBuf::from(path);
    let tests = serde_json::from_str::<TestCases>(
        &fs::read_to_string(&p).expect("failed to open test data"),
    )
    .expect("failed to parse test data");

    let runtime = Rc::new(RefCell::new(None));

    let mut recreate_runtime: Box<dyn Fn()> = Box::new(|| {});
    let mut skip = false;
    let mut module_index = -1;
    let total_tests = tests.commands.len();

    for (test_i, test) in tests.commands.into_iter().enumerate() {
        recreate_runtime();

        match test {
            Case::Module(module) => {
                let runtime = runtime.clone();
                let mut p = p.clone();
                p.pop();
                p.push(&module.filename);
                skip = module
                    .filename
                    .extension()
                    .map(|s| s == "wat")
                    .unwrap_or_default();
                module_index += 1;
                if skip {
                    continue;
                }
                recreate_runtime = Box::new(move || {
                    *runtime.borrow_mut() =
                        Some(Runtime::new(p.clone()).expect("failed to load module"));
                });
            }
            Case::AssertReturn(AssertReturn {
                action, expected, ..
            }) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");

                let expected = const_to_val(expected);

                handle_action(rt, action, move |rt, field| {
                    let mut last;
                    loop {
                        last = rt.stack.first().expect("no first").stack.clone();
                        match rt.step() {
                            Err(RuntimeError::NoFrame(_, _, _)) => {
                                let expected = remove_floats(expected);
                                last = remove_floats(last);
                                if last == expected {
                                    break;
                                } else {
                                    error!("test {test_i}/{total_tests} failed (module: {module_index}, invoke: {field:?}, got {last:?}, but expected {expected:?})");
                                    std::process::exit(1);
                                }
                            }
                            Err(e) => {
                                error!("{e:?} ({:?})", rt.path);
                                std::process::exit(1);
                            }
                            Ok(()) => (),
                        }
                    }
                })
            }
            Case::Action(ActionWrap { action, .. }) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                handle_action(rt, action, move |rt, _| loop {
                    match rt.step() {
                        Err(RuntimeError::NoFrame(_, _, _)) => {
                            break;
                        }
                        Err(e) => {
                            error!("{e:?}");
                            std::process::exit(1);
                        }
                        Ok(()) => (),
                    }
                })
            }
            Case::AssertExhaustion(assert_exhaustion) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertExhaustion")
            }
            Case::AssertTrap(AssertTrap { action, text, .. }) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                handle_action(rt, action, move |rt, field| loop {
                    match rt.step() {
                        Err(RuntimeError::NoFrame(_, _, _)) => {
                            error!("test {test_i}/{total_tests} did not fail, expected error: {text:?} (module: {module_index}, function {field:?})");
                            std::process::exit(1);
                        }
                        Err(e) if format!("{e:?}") == text => {
                            break;
                        }
                        Err(e) => {
                            error!("{e:?}");
                            std::process::exit(1);
                        }
                        Ok(()) => (),
                    }
                })
            }
            Case::AssertInvalid(AssertInvalid {
                _type,
                filename,
                text,
                module_type,
            }) => {
                if skip {
                    continue;
                }
                if let ModuleType::Text = module_type {
                    continue;
                }

                let mut p = p.clone();
                p.pop();
                p.push(&filename);

                match Runtime::new(&p) {
                    Ok(_) => {
                        error!("test {test_i}/{total_tests} did not fail invalidating, expected error: {text:?} (module: {p:?})");
                        std::process::exit(1);
                    }
                    Err(_) => continue,
                }
            }
            Case::AssertMalformed(AssertMalformed {
                filename,
                text,
                module_type,
                ..
            }) => {
                if skip && filename.extension().map(|x| x == "wat").unwrap_or_default() {
                    continue;
                }
                // let mut rt = runtime.borrow_mut();
                // let rt = rt.as_mut().expect("no rt set");
                if let ModuleType::Text = module_type {
                    continue;
                }
                todo!("AssertMalformed")
            }
            Case::AssertUninstantiable(assert_uninstantiable) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertUninstantiable")
            }
            Case::AssertUnlinkable(assert_unlinkable) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("AssertUnlinkable")
            }
            Case::Register(register) => {
                if skip {
                    continue;
                }
                let mut rt = runtime.borrow_mut();
                let rt = rt.as_mut().expect("no rt set");
                todo!("Register")
            }
        }
    }
}
