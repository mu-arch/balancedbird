//GLOBALS

let maxToWt = 0;
let maxLdgWt = 0;

function setWtLimits(to,ldg) {
    maxToWt = to;
    maxLdgWt = ldg;
}

function safeDivide(a, b) {
    if (b === 0) {
        return 0;
    }
    return a / b;
}

function trimTrailingZeros(value) {
    if (value.includes('.')) {
        while (value.endsWith('0')) {
            value = value.substring(0, value.length - 1);
        }
        if (value.endsWith('.')) {
            value = value.substring(0, value.length - 1);
        }
    }
    return value;
}

function parseStrict(value) {
    const num = parseFloat(value);
    return value === num.toString() ? num : NaN;
}

function calc_wb(lastInputElement) {

    let total_wt = 0;
    let total_moment = 0;
    let nan_detected = false;

    const rows = document.querySelectorAll(".wbrows tr");

    for (const row of rows) {

        const firstCellText = row.cells[0].textContent.trim();

        let [wtInput, armInput, momentInput] = [...row.cells].slice(1).map(cell => cell.querySelector("input"));

        //reset
        wtInput.closest("tr").classList = "";

        let wt = wtInput.value.trim() !== "" ? parseStrict(wtInput.value) : null;
        let arm = armInput.value.trim() !== "" ? parseStrict(armInput.value) : null;


        if (firstCellText === "Zero Fuel Wt."
            || firstCellText === "Ramp Wt."
            || firstCellText === "T/O Wt.") {
            if (nan_detected) {
                wtInput.value = "Invalid";
                momentInput.value = "Invalid";
                armInput.value = "Invalid"
            } else {
                wtInput.value = total_wt;
                momentInput.value = total_moment;
                armInput.value = safeDivide(total_moment, total_wt).toFixed(3)
            }
            continue
        }

        let galInput = row.querySelector(".gal-view input");

        if (galInput) {
            if (lastInputElement === wtInput) {
                galInput.value = safeDivide(wt, 6.01).toFixed(3)
            } else if (lastInputElement === galInput) {
                wt = trimTrailingZeros((Number(galInput.value) * 6.01).toFixed(3));
                wtInput.value = wt
                //return calc_wb()
            }
        }


        if (isNaN(wt) || isNaN(arm)) {
            nan_detected = true
            wtInput.closest("tr").classList.add("field-error")
        }


        if (firstCellText === "Start/Taxi Fuel") {
            total_wt -= wt;
            arm = Number(document.getElementById("fuelarm").value)
            armInput.value = arm
        } else {
            total_wt += wt;
        }

        total_moment += wt * arm;
        momentInput.value = (wt * arm).toFixed(3)

    }

}

function recompute(lastInput) {
    setWtLimits(Number(document.getElementById("max_to_wt").value), Number(document.getElementById("max_ldg_wt").value))
    calc_wb(lastInput)
}

// Bind the calculateValues function to the input change events
document.querySelectorAll("input").forEach(function(input) {
    input.addEventListener("input", function(event) {
        recompute(input); // passing the current input as an argument
    });
});

document.getElementById("clear").addEventListener("click", function() {
    document.querySelectorAll("input").forEach(function(input) {
        input.value = "";
    });
});



// clear all inputs on page load
/*
document.querySelectorAll("input").forEach(function(input) {
    input.value = "";
});*/