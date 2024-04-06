import init, { calculate } from './pkg/web_calc.js'

init().then(() => {
  console.log('initalized');
})

document.getElementById('calcBtn').addEventListener('click', function() {
  var input = document.getElementById('calcInput').value
  var content = calculate(input)
  if (content === undefined) {
    console.log('undefined');
    resultText.textContent = "error ";
    resultText.style.color = "red";
  } else {
    console.log(`result: ${content}`);
    resultText.textContent = "Result: " + content;
    resultText.style.color = "black";
  }
})
