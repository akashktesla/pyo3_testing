#![allow(warnings)]

use pyo3::{prelude::*, py_run};
use pyo3::prelude::PyModule;
use pyo3::types::{PyTuple, PyList};

use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let py_app = include_str!("/home/aki/projects/pyo3_testing/aki.py");
    let  from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("blabla")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}


fn run_python()->PyResult<()>{
    let arg1 = "hellow1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
                    "",
                    "",
                    )?
            .getattr("example")?
            .into();

        // call object without any arguments
        fun.call0(py)?;

        // call object with PyTuple
        let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        fun.call1(py, args)?;

        // pass arguments as rust tuple
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;
        Ok(())
    })

}

fn spacy_testing() {
    Python::with_gil(|py| {
        let spacy = PyModule::import(py, "spacy").unwrap();
        let args = PyTuple::new(py, &["en_core_web_sm"]);
        let load = spacy.getattr("load").unwrap();
        let nlp = load.call1(args).unwrap();
        println!("nlp: {:?}",nlp);
        let args = PyTuple::new(py, &["I love to dominate the world. I don't give a fuck"]);
        let doc = nlp.call1(args).unwrap();
        println!("doc: {:?}",doc);
        let sents= doc.getattr("sents").unwrap().iter().unwrap();
        println!("sents: {:?}",sents);
        for i in sents.into_iter(){
            // println!("i: {:?}",i.unwrap());
            let sent = i.unwrap();
            for token in sent.iter().unwrap().into_iter(){
                let token = token.unwrap();
                println!("token: {:?}",token);
                let head = token.getattr("head").unwrap();
                let head_pos = head.getattr("pos_").unwrap();
                println!("head: {:?}",head);
            }
        }
    })
}

fn spacy_testing2(){
    Python::with_gil(|py|{
        let my_module = PyModule::new(py, "aki").unwrap();
        let sys_modules = py.import("sys").unwrap().getattr("modules").unwrap();
        sys_modules.set_item("aki", my_module).unwrap();
        py.run(
            r#"

from aki import *
            "#
            ,
            None,
            None
        ).unwrap();
    })
}


