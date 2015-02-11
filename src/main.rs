#![feature(os,path,io,collections)]
use std::old_io::File;
use std::os;
use std::old_io::BufferedReader;

#[warn(dead_code)]
enum Protien { G,P,A,V,L,I,M,C,F,Y,W,H,K,R,Q,N,E,D,S,T,X,STOP }

#[warn(dead_code)]
struct FASTAQ {
	file: BufferedReader<std::old_io::fs::File>,
	next: usize,
}

#[warn(dead_code)]
struct Record {
	name: String,
	error: f32,
	seq: Vec<Protien>,
}

fn translate(three: &[char]) -> char {
	match three {
		['T','C','A'] => 'S',    // Serine
		['T','C','C'] => 'S',    // Serine
		['T','C','G'] => 'S',    // Serine
		['T','C','T'] => 'S',    // Serine
		['T','T','C'] => 'F',    // Phenylalanine
		['T','T','T'] => 'F',    // Phenylalanine
		['T','T','A'] => 'L',    // Leucine
		['T','T','G'] => 'L',    // Leucine
		['T','A','C'] => 'Y',    // Tyrosine
		['T','A','T'] => 'Y',    // Tyrosine
		['T','A','A'] => '_',    // Stop
		['T','A','G'] => '_',    // Stop
		['T','G','C'] => 'C',    // Cysteine
		['T','G','T'] => 'C',    // Cysteine
		['T','G','A'] => '_',    // Stop
		['T','G','G'] => 'W',    // Tryptophan
		['C','T','A'] => 'L',    // Leucine
		['C','T','C'] => 'L',    // Leucine
		['C','T','G'] => 'L',    // Leucine
		['C','T','T'] => 'L',    // Leucine
		['C','C','A'] => 'P',    // Proline
		['C','C','C'] => 'P',    // Proline
		['C','C','G'] => 'P',    // Proline
		['C','C','T'] => 'P',    // Proline
		['C','A','C'] => 'H',    // Histidine
		['C','A','T'] => 'H',    // Histidine
		['C','A','A'] => 'Q',    // Glutamine
		['C','A','G'] => 'Q',    // Glutamine
		['C','G','A'] => 'R',    // Arginine
		['C','G','C'] => 'R',    // Arginine
		['C','G','G'] => 'R',    // Arginine
		['C','G','T'] => 'R',    // Arginine
		['A','T','A'] => 'I',    // Isoleucine
		['A','T','C'] => 'I',    // Isoleucine
		['A','T','T'] => 'I',    // Isoleucine
		['A','T','G'] => 'M',    // Methionine
		['A','C','A'] => 'T',    // Threonine
		['A','C','C'] => 'T',    // Threonine
		['A','C','G'] => 'T',    // Threonine
		['A','C','T'] => 'T',    // Threonine
		['A','A','C'] => 'N',    // Asparagine
		['A','A','T'] => 'N',    // Asparagine
		['A','A','A'] => 'K',    // Lysine
		['A','A','G'] => 'K',    // Lysine
		['A','G','C'] => 'S',    // Serine
		['A','G','T'] => 'S',    // Serine
		['A','G','A'] => 'R',    // Arginine
		['A','G','G'] => 'R',    // Arginine
		['G','T','A'] => 'V',    // Valine
		['G','T','C'] => 'V',    // Valine
		['G','T','G'] => 'V',    // Valine
		['G','T','T'] => 'V',    // Valine
		['G','C','A'] => 'A',    // Alanine
		['G','C','C'] => 'A',    // Alanine
		['G','C','G'] => 'A',    // Alanine
		['G','C','T'] => 'A',    // Alanine
		['G','A','C'] => 'D',    // Aspartic Acid
		['G','A','T'] => 'D',    // Aspartic Acid
		['G','A','A'] => 'E',    // Glutamic Acid
		['G','A','G'] => 'E',    // Glutamic Acid
		['G','G','A'] => 'G',    // Glycine
		['G','G','C'] => 'G',    // Glycine
		['G','G','G'] => 'G',    // Glycine
		['G','G','T'] => 'G',    // Glycine
		_ => '_',
	}
}
// fn DNAtoProtien(threeDNA: typ) -> ret {
// 	// add code here
// }
// impl FASTAQ {
// 	fn new(reader: BufferedReader<std::old_io::fs::File>) -> FASTAQ {
// 		Monster
// 	}
// }

// impl Iterator for FASTAQ {
//   fn next(&mut self) -> Record {
//   	let lines_iter = &self.file.lines().filter_map(|result| result.ok())
//   	let next_four = lines_iter.take(4);
//   }
// }


fn main() {
	let args: Vec<String> = os::args();
	let (path_in, path_out) = match (args.get(1), args.get(2)) {
		(Some(file_in), Some(file_out)) => (Path::new(file_in), Path::new(file_out)),
		_ => panic!("Pass input and output files"),
	};

	let file_in = match File::open(&path_in) {
		Err(why) => panic!("couldn't open {}: {}", path_in.display(), why.desc),
		Ok(file) => file,
	};

	let mut file_out = match File::create(&path_out) {
			Err(why) => panic!("couldn't create {}: {}", path_out.display(), why.desc),
			Ok(file) => file,
	};

	let mut reader = BufferedReader::new(file_in);
	let mut lines = reader.lines().filter_map(|result| result.ok());
	loop {
		match (lines.next(), lines.next(), lines.next(), lines.next()) {
			(Some(name), Some(seq), _, Some(error)) => {
				// let mut protien_sequence:Vec<char> = vec![];
				let mut protien_sequence: String = "".to_string();
				let bytes: Vec<char> = seq.chars().collect();
				let mut partitioned = bytes.chunks(3);
				for three in partitioned {
					protien_sequence.push(translate(three));
				}
				protien_sequence.push_str("\n");
				match file_out.write_str(&protien_sequence) {
					Err(why) =>  panic!("couldn't write to {}", why.desc),
					Ok(_) => println!("successfully wrote to"),
				}
			},
			_ => break,
		}
	};
}