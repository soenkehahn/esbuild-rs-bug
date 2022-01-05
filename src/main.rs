use esbuild_rs::*;
use std::sync::{mpsc::channel, Arc};

fn main() {
    let mut builder = BuildOptionsBuilder::new();
    builder.platform = Platform::Node;
    builder.entry_points = vec!["main.js".to_owned()];
    builder.bundle = true;
    builder.outfile = "bundled.js".to_owned();
    builder.write = true;
    build_direct_sync(builder.build());
}

fn build_direct_sync(options: Arc<BuildOptions>) -> BuildResult {
    let (sender, receiver) = channel();
    build_direct(options, move |build_result| {
        sender.send(build_result).unwrap()
    });
    let build_result = receiver.recv().unwrap();
    for error in build_result.errors.as_slice() {
        panic!("bundle error: {}", error.text.as_str());
    }
    build_result
}
