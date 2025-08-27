use bob::*;

#[test]
fn asking_a_question() {
	let remark = "How are you?";
	assert_eq!(reply(remark), "Sure.");
}

#[test]
fn yelling() {
	let remark = "WHAT THE HECK";
	assert_eq!(reply(remark), "Whoa, chill out!");
}

#[test]
fn yelling_question() {
	let remark = "WHAT THE HECK?";
	assert_eq!(reply(remark), "Calm down, I know what I'm doing!");
}

#[test]
fn silence() {
	let remark = "    \t";
	assert_eq!(reply(remark), "Fine. Be that way!");
}

#[test]
fn regular() {
	let remark = "What the heck";
	assert_eq!(reply(remark), "Whatever.");
}

#[test]
fn non_letters_with_question() {
	let remark = ":) ?";
	assert_eq!(reply(remark), "Sure.");
}

#[test]
fn no_letters() {
	let remark = "1, 2, 3";
	assert_eq!(reply(remark), "Whatever.");
}

#[test]
fn question_with_no_letters() {
	let remark = "4?";
	assert_eq!(reply(remark), "Sure.");
}
