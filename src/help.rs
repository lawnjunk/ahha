pub fn help() {
    let version = env!("CARGO_PKG_VERSION").to_string();
    println!(
        "Ahha (v{}) - a minimal commandline utility for storing your ideas\n",
        version
    );
    println!("USAGE: ahha (optional flags)\n");
    println!("OPTIONS:");
    println!("  -h, --help        print this help");
    println!("  -s, --sync        sync ahha with remote");
    println!("      --clone       clone an existing repo");
}
