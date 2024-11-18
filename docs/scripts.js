//GLOBALS

let maxToWt = 0;
let maxLdgWt = 0;


function setWtLimits(to, ldg) {
    maxToWt = to;
    maxLdgWt = ldg;
}

function safeDivide(a, b) {
    if (b === 0) {
        return 0;
    }
    return a / b;
}

function roundToTwo(num) {
    return +(Math.round(num + "e+2") + "e-2");
}

function w(element, value) {
    element.value = roundToTwo(Number(value))
}

function parseStrict(value) {
    if (value.endsWith('.')) {
        value = value.slice(0, -1);  // Remove the trailing period
    }
    const num = parseFloat(value);
    return value === num.toString() ? num : NaN;
}

function collect_wb_row(row) {
    let [wtInput, armInput, momentInput] = [...row.cells].slice(1).map(cell => cell.querySelector("input"));

    wtInput.closest("tr").classList = "";

    let wt = wtInput.value.trim() !== "" ? parseStrict(wtInput.value) : null;
    let arm = armInput.value.trim() !== "" ? parseStrict(armInput.value) : null;

    console.log(wt, arm)

    if (isNaN(wt) || isNaN(arm)) {
        wtInput.closest("tr").classList.add("field-error")
    }

    return [wtInput, armInput, momentInput, wt, arm]
}

function processGalInput(row, lastInputElement, wtInput, wt) {
    let galInput = row.querySelector(".gal-view input");

    if (lastInputElement === wtInput) {
        w(galInput, safeDivide(wt, 6.01));
        return false; // Continue execution in the outer function
    } else if (lastInputElement === galInput) {
        let gal = galInput.value.trim() !== "" ? parseStrict(galInput.value) : null;
        wt = (gal * 6.01);
        wtInput.value = wt;
        return true; // Exit the outer function
    }
    return false; // Default behavior: continue execution in the outer function
}


function calc_wb(lastInputElement) {

    let total_wt = 0;
    let total_moment = 0;
    let fuel_arm = 0;
    let fuel_gallons = 0;

    const rows = document.querySelectorAll(".wbrows tr");

    for (const row of rows) {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(row);

        wtInput.closest("tr").classList = "";
    }

    for (const row of rows) {
        let firstTd = row.querySelector('td').textContent;
        if (firstTd === "Zero Fuel Wt.") {
            break
        }
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(row);

        total_wt += wt;
        total_moment += wt * arm;
        w(momentInput, (wt * arm))
    }

    //zero fuel weight row
    {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(rows[6]);
        w(wtInput, total_wt)
        w(momentInput, total_moment)
        w(armInput, safeDivide(total_moment, total_wt))
    }

    //fuel
    {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(rows[7]);

        if (processGalInput(rows[7], lastInputElement, wtInput, wt)) {
            return calc_wb(null);
        }

        let galInput = rows[7].querySelector(".gal-view input");
        fuel_gallons = galInput.value.trim() !== "" ? parseStrict(galInput.value) : null;


        fuel_arm = arm;

        let moment = wt * arm;
        w(momentInput, moment)
        total_wt += wt
        total_moment += moment
    }

    //ramp wt
    {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(rows[8]);
        w(wtInput, total_wt)
        w(momentInput, total_moment)
        w(armInput, safeDivide(total_moment, total_wt))
    }

    {
        let [wtInput, armInput, momentInput, wt, _] = collect_wb_row(rows[9]);

        if (processGalInput(rows[9], lastInputElement, wtInput, wt)) {
            return calc_wb(null);
        }

        let galInput = rows[9].querySelector(".gal-view input");
        let fuel_used = galInput.value.trim() !== "" ? parseStrict(galInput.value) : null;

        let arm = fuel_arm;

        let moment = wt * arm;
        w(armInput, arm)
        w(momentInput, moment)
        total_wt -= wt
        total_moment -= moment
        fuel_gallons -= fuel_used
    }

    {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(rows[10]);
        w(wtInput, total_wt)
        w(momentInput, total_moment)
        w(armInput, safeDivide(total_moment, total_wt))
    }

    {
        let gph = parseStrict(document.getElementById("burnrate").value);
        let hours = parseStrict(document.getElementById("time").value);

        document.getElementById("used").value = (hours * gph);
        document.getElementById("remain").value = fuel_gallons - (hours * gph)

        let [wtInput, armInput, momentInput, _5, _6] = collect_wb_row(rows[11]);

        let arm = fuel_arm;

        let burnwt = (hours * gph) * 6.01
        let moment = (burnwt * arm);
        w(wtInput, burnwt)
        w(momentInput, moment)
        w(armInput, arm)

        total_wt -= burnwt
        total_moment -= moment
    }

    {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(rows[12]);
        w(wtInput, total_wt)
        w(momentInput, total_moment)
        w(armInput, safeDivide(total_moment, total_wt))
    }


    //final run through which checks again for NaN. this is needed since some rows are edited out of order
    for (const row of rows) {
        let [wtInput, armInput, momentInput, wt, arm] = collect_wb_row(row);
    }


}

function recompute(lastInput) {
    setWtLimits(Number(document.getElementById("max_to_wt").value), Number(document.getElementById("max_ldg_wt").value))
    calc_wb(lastInput)
}

// Bind the calculateValues function to the input change events
document.querySelectorAll("input").forEach(function (input) {
    input.addEventListener("input", function (event) {
        recompute(input); // passing the current input as an argument
    });
});

document.getElementById("clear").addEventListener("click", function () {
    document.querySelectorAll("input").forEach(function (input) {
        input.value = "";
    });
});


// clear all inputs on page load
/*
document.querySelectorAll("input").forEach(function(input) {
    input.value = "";
});*/

// Get all input fields in the document
const inputFields = document.querySelectorAll('input');

// Function to strip '-' and ',' from the input value
function stripCharacters(event) {
    // Get the current value of the input field
    let currentValue = event.target.value;

    // Strip '-' and ',' from the value
    let newValue = currentValue.replace(/[-,]/g, '');

    // Set the new value back to the input field
    event.target.value = newValue;
}

// Add the event listener to each input field
inputFields.forEach(inputField => {
    inputField.addEventListener('input', stripCharacters);
});

function selectPreset(element) {
    document.getElementById('presets').querySelectorAll(':scope > div').forEach(child => {
        child.classList.remove('active');
    });

    element.classList.add('active');
}

function checkEnter(event) {
    if (event.key === 'Enter') {  // If Enter key is pressed
        event.preventDefault();  // Prevent form submission (default behavior)
        submitAirportId();  // Call the submit function directly
    }
}

function submitAirportId() {
    const airportId = document.getElementById('weather-input').value;  // Get the input value
    document.getElementById('weather-table').style.display = "none";
    document.getElementById('weather-text').style.display = "block";

    let loader ="<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1em\" height=\"1em\" viewBox=\"0 0 24 24\"><rect width=\"7.33\" height=\"7.33\" x=\"1\" y=\"1\" fill=\"currentColor\"><animate id=\"svgSpinnersBlocksWave0\" attributeName=\"x\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"y\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"width\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"8.33\" y=\"1\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"1\" y=\"8.33\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"15.66\" y=\"1\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"8.33\" y=\"8.33\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"1\" y=\"15.66\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"15.66\" y=\"8.33\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"8.33\" y=\"15.66\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"15.66\" y=\"15.66\" fill=\"currentColor\"><animate id=\"svgSpinnersBlocksWave1\" attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect></svg>";

    document.getElementById('weather-text').innerHTML = loader + " Please wait. Contacting FAA systems...";

    const randomDelay = Math.floor(Math.random() * (700 - 100 + 1)) + 500;

    setTimeout(function() {
        document.getElementById('weather-text').innerHTML =loader + " Downloading latest weather observation from aviationweather.gov......";

        setTimeout(function() {
            document.getElementById('weather-table').style.display = "block";
            document.getElementById('weather-text').style.display = "none";

        }, randomDelay);
    }, randomDelay);

    let metar_error = "Something went wrong. The weather system may be down or the server is not responding."
}