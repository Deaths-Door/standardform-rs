import init , { StandardForm } from './pkg/standardform.js';

await init()

document.querySelector('button').addEventListener('click',() => {
    var number = document.getElementById('convertor_button').value;
    var result = document.getElementById('result');
    var title = document.getElementById('result-title');

    // TODO : Convert number
    try { 
        result.innerText = StandardForm.new_from_string(number).into_f64() 
    } catch(err) {
        result.innerText = "";
        title.innerText = err;
        title.style.color = 'red';
        setTimeout(() => {
            title.style.color = "black";
            title.innerText = "Result"
        },3000);
    }
})