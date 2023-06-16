use std::{collections::HashMap, default};

#[derive(Debug)]
pub struct Names {
    long : String,
    short : String
}
type Id = String;
type Description = String;

#[derive(Debug)]
pub enum Value<T> {
    Flag(bool),
    Value(T)
}

#[derive(Debug)]
pub struct Argument<T,E>{
    key : Id,
    value : Result<Value<T>,E>,
}

#[derive(Debug)]
pub enum ArgumentError<E> {
    UnrecognizedArgument(String),
    AcceptanceError(E)
}

#[derive(Debug)]
pub struct Parser <ArgType,ErrType>{
    description : String,
    args : Vec<(Id,Names,Description,Option<fn(String) -> Result<ArgType,ErrType>>,Option<Value<ArgType>>)>
}
impl<ArgType:std::fmt::Debug,ErrType:std::fmt::Debug> Parser<ArgType,ErrType> {
    pub fn build(description:&str) -> Self {
        let mut args = Vec::new();
        args.push(("help".to_string(),Names{long:"--help".to_string(),short:"-h".to_string()},"display help information".to_string(),None,None));
        Self {  
            description : description.to_string(),
            args : args,
        }
    }
    pub fn print_help(&self) {
        println!("\x1b[1;34m{}\x1b[0m", self.description);
        for (name,Names { long, short },desc,accept,default) in &self.args {
            let is_flag = if accept.is_none()  {"\x1b[1;37;45m FLAG \x1b[0m"} else {""};

            let default = 
                if let Some(value) = default {
                    match value {
                        Value::Flag(f) => format!("\n\t\tdefault = \x1b[1;35m{}\x1b[0m",f),
                        Value::Value(v) => format!("\n\t\tdefault = \x1b[1;33m{:?}\x1b[0m",v),
                    }
                }else {
                    "".to_string()
                }
            ;
            println!("\t\x1b[1;36m{0}\x1b[0m : \x1b[2;33m{1}\x1b[0m,\x1b[2;33m{2}\x1b[0m {3} {4}{5}",name,long,short,desc,is_flag,default)
        }
    }
    pub fn arg(&mut self,long_name:&str, short_name:&str, description:&str,accept:fn(String) -> Result<ArgType,ErrType> ) -> &mut Self {
        let names = Names { long : format!("--{}",long_name).to_string() , short : format!("-{}",short_name).to_string()};
        self.args.push((long_name.to_string(),names,description.to_string(),Some(accept),None));
        self
    }
    pub fn arg_default(&mut self,long_name:&str, short_name:&str, description:&str,default:ArgType,accept:fn(String) -> Result<ArgType,ErrType> ) -> &mut Self {
        let names = Names { long : format!("--{}",long_name).to_string() , short : format!("-{}",short_name).to_string()};
        self.args.push((long_name.to_string(),names,description.to_string(),Some(accept),Some(Value::Value(default))));
        self
    }
    pub fn flag(&mut self,long_name:&str, short_name:&str, description:&str,default:bool) -> &mut Self {
        let names = Names { long : format!("--{}",long_name).to_string() , short : format!("-{}",short_name).to_string()};
        self.args.push((long_name.to_string(),names,description.to_string(),None,Some(Value::Flag(default))));
        self
    }
    pub fn parse(&self) -> Result<HashMap<Id,Value<ArgType>>,ArgumentError<ErrType>> {
        let mut args = std::env::args();
        let program_name = args.next().unwrap_or("".to_string());

        let mut args : Vec<String> = args.collect();
        
        if args.len() == 1 {
            let first = args.get(0).unwrap().to_string();
            let arg = self.args.iter().find_map(|(id,names,_,accept,default)| {
                if (first == names.long || first == names.short) && accept.is_none() {
                    return Some(Argument::<ArgType, ErrType>{key:id.to_owned(),value:Ok(Value::<ArgType>::Flag(true))});
                }else {
                    return None;
                }
            });
            if let Some(arg) = arg {
                match arg.value {
                    Ok(val) => {
                        let mut res = HashMap::new();
                        res.insert(arg.key, val);
                        return Ok(res)
                    },
                    Err(err) => {
                        return Err(ArgumentError::AcceptanceError(err));
                    }
                }
            }else {
                return Err(ArgumentError::UnrecognizedArgument(first));
            }
        }
        args.push("".to_string());
        
        
        let mut args_next:Vec<String> = {
            let mut a = args.iter().map(|s| s.to_string());
            a.next();
            let collected = a.collect();
            collected
        };
        let args = args.iter().zip(args_next.iter());

        let mut res = HashMap::new();
        let mut skip_next = false;
        
        for arg in args {
            if skip_next {
                skip_next = false;
                continue;
            }
            let first = arg.0.to_string();

            let arg = self.args.iter().find_map(
                |(id,names,_,accept,_)| {
                    if first == names.long || first == names.short {
                        if let Some(accept) = accept {
                            let res = accept(arg.1.to_string());
                            skip_next = true;
                            match res {
                                Ok(val) => {
                                    return Some(Argument{key:id.to_owned(),value:Ok(Value::<ArgType>::Value(val))});
                                },
                                Err(err) => {
                                    return Some(Argument{key:id.to_owned(),value:Err(err)});
                                }
                            }
                        }else {
                            skip_next = false;
                            return Some(Argument{key:id.to_owned(),value:Ok(Value::<ArgType>::Flag(true))});
                        }
                    }else {
                        return None;
                    }
            });
            if let Some(arg) = arg {
                match arg.value {
                    Ok(val) => {
                        res.insert(arg.key, val);
                    },
                    Err(err) => {
                        return Err(ArgumentError::AcceptanceError(err));
                    }
                }
            }else {
                return Err(ArgumentError::UnrecognizedArgument(first));
            }
        }
        Ok(res)
    }
}
