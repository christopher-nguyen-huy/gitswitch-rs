use std::{io, fs, process, env};
use std::process::Command;
use std::path::Path;

#[cfg(target_family = "unix")]
use std::os::unix::fs as unixfs; // fs::symlink

#[cfg(windows)]
use std::os::windows::fs as winfs; // fs::symlink_file(src, dest)

use csv;
use home;
use serde::Deserialize;

const ACCOUNTS_FILE: &str = "accounts.csv";

#[derive(Deserialize)]
struct Account {
	key: String,
	name: String,
	email: String,
}

fn main() -> std::io::Result<()> {
	check_git();
	let home_dir = home::home_dir().unwrap();
	check_ssh(&home_dir);
	let ssh_dir = home_dir.join(".ssh");
	check_accounts(&ssh_dir);

	env::set_current_dir(&ssh_dir)?;
	let accounts_dir = Path::new("accounts");
	let pub_dest_path = Path::new("id_rsa.pub");
	let priv_dest_path = Path::new("id_rsa");
	
	//https://stackoverflow.com/q/49283092
	match fs::read_link(&priv_dest_path) {
		Ok(currentlink) => println!("Current key: {:?}", currentlink),
		Err(_) => println!("SSH key not set"),
	}
	
	let accounts = load_accounts_file();
	for (i, account) in accounts.iter().enumerate() {
		println!("[{}]: {}", i + 1, account.key)
	}
	println!("[q]: Quit");

	let mut input = String::new();
	io::stdin().read_line(& mut input).expect("Readline error");
	input = String::from(input.trim_end()); // remove newline at the end

	let mut num: usize;
	match input.as_str() {
		"q" => {
			process::exit(0);
		},
		_ => {
			num = input.parse().expect("Not an integer...");
			let maxlen = accounts.len();
			if num < 1 || num > maxlen {
				println!("Out of range selection");
				process::exit(1);
			}
		},
	}

	num -= 1;
	let priv_src_path = accounts_dir.join(&accounts[num].key);
	let pub_src_path = accounts_dir.join(format!("{}.pub",&accounts[num].key));
	if (&priv_dest_path).exists() {
		fs::remove_file(&priv_dest_path).expect("Could not delete current symlinked private key.");
	}
	if (&pub_dest_path).exists() {
		fs::remove_file(&pub_dest_path).expect("Could not delete current symlinked public key.");
	}
	create_symlink(&priv_src_path, &priv_dest_path, "Could not create private key symlink.");
	create_symlink(&pub_src_path, &pub_dest_path, "Could not create public key symlink.");

	Command::new("git")
		.args(&["config", "--global", "user.email"])
		.arg(&accounts[num].email)
		.output()
		.expect("Error changing git email");
	Command::new("git")
		.args(&["config", "--global", "user.name"])
		.arg(&accounts[num].name)
		.output()
		.expect("Error changing git name");
	Ok(())
}

fn load_accounts_file() -> Vec<Account> {
	let mut csvr = csv::Reader::from_path(Path::new(ACCOUNTS_FILE)).unwrap();
	let mut accounts = Vec::new();
	for result in csvr.deserialize() {
		let account: Account = result.unwrap();
		accounts.push(account)
	}
	accounts
}

#[cfg(target_family = "unix")]
fn create_symlink(src: &Path, dest: &Path, err: &str) {
	unixfs::symlink(src, dest).expect(err);
}

#[cfg(target_os = "windows")]
fn create_symlink(src: &Path, dest: &Path, err: &str) {
	winfs::symlink_file(src, dest).expect(err);
}

// fn load_accounts_dir() -> Result<Vec<Path>, io::Error> {
// 	let mut paths = fs::read_dir(Path::new("accounts"))?
// 		.map(|res| res.map(|e| e.path()))
// 		.filter(|x| x.as_ref().unwrap().to_str().unwrap().ends_with(".pub"))
// 		.collect::<Result<Vec<_>, io::Error>>()?;
// 	paths.sort();
// 	paths
// }

fn check_git() {
	let gitver = Command::new("git")
		.arg("--version")
		.output()
		.expect("Failed to execute git check process");
	let output = String::from_utf8(gitver.stdout).unwrap(); //stdout is Vec<u8>
	if !output.contains("git version") {
		println!("Git not installed.");
		process::exit(1);
	}
}

fn check_ssh(homepath: &Path) {
	if !homepath.join(".ssh").exists() {
		println!("Missing ~/.ssh/ directory.");
		process::exit(1);
	}
}

fn check_accounts(sshpath: &Path) {
	if !sshpath.join(ACCOUNTS_FILE).exists() {
		println!("Missing ~/.ssh/{} file.", ACCOUNTS_FILE);
		process::exit(1);
	}
}