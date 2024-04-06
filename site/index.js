import * as wasm from 'web-calc'

document.getElementById('calcBtn').addEventListener('click', function() {
  var input = document.getElementById('calcInput').value
  console.log(`input string: ${input}`);
  var content = wasm.calculate(input)
  if (content === undefined) {
    console.log('undefined');
    resultText.textContent = "Error, please check and input expression again";
    resultText.style.color = "red";
  } else {
    console.log(`result: ${content}`);
    resultText.textContent = "Result: " + content;
    resultText.style.color = "black";
  }
})
