const fs = require('fs');
let regex = new RegExp(/\d/, "g");

let file = fs.readFileSync('2.txt', 'utf8');
let splitFile = file.split('\n');

let sum = 0;

for (let line of splitFile) {
	let currentLine = line.replaceAll("one", "o1e").replaceAll("two", "t2o").replaceAll("three", "th3ee").replaceAll("four", "fo4r").replaceAll("five", "fi5e").replaceAll("six", "s6x")
		.replaceAll("seven", "se7en").replaceAll("eight", "ei8ht").replaceAll("nine", "ni9e");
	let currentLineMatches = currentLine.match(regex);

	if (!currentLineMatches) continue;
	let wholeNumber = `${currentLineMatches[0]}${currentLineMatches[currentLineMatches.length - 1]}`	

	sum += parseInt(wholeNumber);
}

console.log(sum);
