//GLOBALS

let maxToWt = 0;
let maxLdgWt = 0;

function setWtLimits(to,ldg) {
    maxToWt = to;
    maxLdgWt = ldg;
}

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

function calc_wb_part_2() {
    document.querySelectorAll(".wb2 tr:last-child")[0].style.background = null;
    let totalWeight = 0;
    let totalMoment = 0;
    const rows = document.querySelectorAll(".wb2 tr");
    const zeroFuelWeightInputs = document.querySelectorAll(".wb2 tr:last-child input");
    zeroFuelWeightInputs.forEach(input => input.value = "0");

    {
        let row = rows[0];
        let [weightInput, armInput, momentInput] = [...row.cells].slice(1).map(cell => cell.querySelector("input"));
        let weightVal = weightInput.value;
        let armVal = armInput.value;

        if (weightVal === "") weightVal = "0";
        if (armVal === "") armVal = "0";

        const weight = Number(weightInput.value);
        const arm = Number(armInput.value);

        if (isNaN(weight) || isNaN(arm)) {
            zeroFuelWeightInputs.forEach(input => input.value = "Invalid input");
            document.querySelectorAll(".wb2 tr:last-child")[0].style.background = "#ffd3d3";
            return; // Exit the function early if NaN is detected
        }

        const moment = weight * arm;
        momentInput.value = moment % 1 === 0 ? moment : moment.toFixed(2);

        totalWeight += weight;
        totalMoment += moment;
    }
    {
        let row = rows[1];
        let [weightInput, armInput, momentInput] = [...row.cells].slice(1).map(cell => cell.querySelector("input"));
        let [zeroFuelWt, zeroFuelArm, zeroFuelMoment] = document.querySelectorAll(".wb1 tr:last-child input");
        weightInput.value = Math.round(Number(zeroFuelWt.value) + totalWeight);
        momentInput.value = Number(zeroFuelMoment.value) + totalMoment;

        armInput.value = safeDivide(momentInput.value, weightInput.value).toFixed(2);
    }
    {
        let [fuelWt, fuelArm, fuelMoment] = [...rows[0].cells].slice(1).map(cell => cell.querySelector("input"));
        let [taxiUseWtInput, taxiUseArmInput, taxiUseMomentInput] = [...rows[2].cells].slice(1).map(cell => cell.querySelector("input"));
        taxiUseArmInput.value =  fuelArm.value
        taxiUseMomentInput.value = Number(taxiUseWtInput.value) * Number(taxiUseArmInput.value)
    }
    {
        let [rampWeightInput, RampArmInput, rampMomentInput] = [...rows[1].cells].slice(1).map(cell => cell.querySelector("input"));
        let [taxiUseWtInput, taxiUseArmInput, taxiUseMomentInput] = [...rows[2].cells].slice(1).map(cell => cell.querySelector("input"));

        let takeoffWt = Number(rampWeightInput.value) - Number(taxiUseWtInput.value);
        let takeoffMoment = Number(rampMomentInput.value) - Number(taxiUseMomentInput.value);
        let takeoffArm = safeDivide(takeoffMoment, takeoffWt)

        zeroFuelWeightInputs[0].value = takeoffWt;
        //weird parseFloat thing is used to fix formatting of 0.00 to just 0
        zeroFuelWeightInputs[1].value = parseFloat(takeoffArm).toFixed(2).toString();
        zeroFuelWeightInputs[2].value = parseFloat(takeoffMoment.toFixed(2)).toString();
    }

}


function recompute() {
    setWtLimits(Number(document.getElementById("max_to_wt").value), Number(document.getElementById("max_ldg_wt").value))

    calc_wb_part_1()
    calc_wb_part_2()
}

// Bind the calculateValues function to the input change events
document.querySelectorAll("input").forEach(function(input) {
    input.addEventListener("input", recompute);
});

document.getElementById("clear").addEventListener("click", function() {
    document.querySelectorAll("input").forEach(function(input) {
        input.value = "";
    });
});



// clear all inputs on page load
document.querySelectorAll("input").forEach(function(input) {
    input.value = "";
});