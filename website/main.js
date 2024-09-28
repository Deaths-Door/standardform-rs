import init , { StandardForm } from './pkg/standardform.js';

await init()

document.querySelector('button').addEventListener('click',() => {
    var number = document.getElementById('convertor_button').value;
    var result = document.getElementById('result');
    var title = document.getElementById('result-title');

    // TODO : Convert number
    try { 
        const sf = StandardForm.new_from_string(number).into_f64();
        result.innerText = sf;

        let history =  document.getElementById('history');
        const text = number + " = " + sf;

        if (history.lastElementChild?.textContent == text){
            return
        }

        let item = document.createElement("li");

        const spanCreator = (part,sep) => {
            var span = document.createElement('span');
            span.textContent = part;

            if (sep) {
                span.classList.add('separator');
            }

            item.appendChild(span);   
        };
        
        spanCreator(number,false);
        item.appendChild(document.createTextNode(' = '),true);
        spanCreator(sf,false);
    
        history.appendChild(item)
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