document.getElementById("clear").addEventListener("click", function() {
    document.querySelectorAll("input").forEach(function(input) {
        input.value = "";
    });
});

function safeDivide(a, b) {
    if (b === 0) {
        return 0; // or return any value you prefer
    }
    return a / b;
}


function calc_wb_part_1() {
    document.querySelectorAll(".wb1 tr:last-child")[0].style.background = null;
    let totalWeight = 0;
    let totalMoment = 0;
    const rows = document.querySelectorAll(".wb1 tr:not(:last-child)");
    const zeroFuelWeightInputs = document.querySelectorAll(".wb1 tr:last-child input");
    zeroFuelWeightInputs.forEach(input => input.value = "0");

    for (const row of rows) {
        let [weightInput, armInput, momentInput] = [...row.cells].slice(1).map(cell => cell.querySelector("input"));
        let weightVal = weightInput.value;
        let armVal = armInput.value;

        if (weightVal === "") weightVal = "0";
        if (armVal === "") armVal = "0";

        const weight = Number(weightInput.value);
        const arm = Number(armInput.value);

        if (isNaN(weight) || isNaN(arm)) {
            zeroFuelWeightInputs.forEach(input => input.value = "Invalid input");
            document.querySelectorAll(".wb1 tr:last-child")[0].style.background = "#ffd3d3";
            return; // Exit the function early if NaN is detected
        }

        const moment = weight * arm;
        momentInput.value = moment % 1 === 0 ? moment : moment.toFixed(2);

        totalWeight += weight;
        totalMoment += moment;
    }


    zeroFuelWeightInputs[0].value = Math.round(totalWeight);
    //weird parseFloat thing is used to fix formatting of 0.00 to just 0
    zeroFuelWeightInputs[1].value = parseFloat(safeDivide(totalMoment, totalWeight).toFixed(2)).toString();
    zeroFuelWeightInputs[2].value = parseFloat(totalMoment.toFixed(2)).toString();
}


// Bind the calculateValues function to the input change events
document.querySelectorAll(".wb1 input").forEach(function(input) {
    input.addEventListener("input", calc_wb_part_1);
});


// clear all inputs on page load
document.querySelectorAll("input").forEach(function(input) {
    input.value = "";
});