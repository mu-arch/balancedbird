document.getElementById("clear").addEventListener("click", function() {
    document.querySelectorAll("input").forEach(function(input) {
        input.value = "";
    });
});


document.querySelectorAll('input[type="number"]').forEach(function(input) {
    input.addEventListener('keydown', function(e) {
        // Allow: backspace, delete, tab, escape, enter, period, and decimal point
        if ([46, 8, 9, 27, 13, 110, 190].indexOf(e.keyCode) !== -1 ||
            // Allow: Ctrl+A
            (e.keyCode === 65 && e.ctrlKey === true) ||
            // Allow: Ctrl+C
            (e.keyCode === 67 && e.ctrlKey === true) ||
            // Allow: Ctrl+X
            (e.keyCode === 88 && e.ctrlKey === true) ||
            // Allow: home, end, left, right
            (e.keyCode >= 35 && e.keyCode <= 39)) {
            // Let it happen, don't do anything
            return;
        }
        // Ensure that it is a number or a period, and stop the keypress if not
        if ((e.shiftKey || (e.keyCode < 48 || e.keyCode > 57)) &&
            (e.keyCode < 96 || e.keyCode > 105) && e.keyCode !== 190) {
            e.preventDefault();
        }
    });
});


function calc_wb_part_1() {
    // Iterate through each row except the Zero Fuel Weight row
    var totalWeight = 0;
    var totalMoment = 0;
    var rows = document.querySelectorAll(".wb1 tr:not(:last-child)");
    rows.forEach(function(row) {
        var weightInput = row.cells[1].querySelector("input");
        var armInput = row.cells[2].querySelector("input");
        var momentInput = row.cells[3].querySelector("input");

        var weight = parseFloat(weightInput.valueAsNumber) || 0;
        var arm = parseFloat(armInput.valueAsNumber) || 0;
        var moment = weight * arm;

        momentInput.value = moment % 1 === 0 ? moment : moment.toFixed(2);


        totalWeight += weight;
        totalMoment += moment;
    });

    // Set the Zero Fuel Weight row values
    document.querySelector("tr:last-child td:nth-child(2) input").value = Math.round(totalWeight);
    document.querySelector("tr:last-child td:nth-child(3) input").value = (totalMoment / totalWeight).toFixed(2);
    document.querySelector("tr:last-child td:nth-child(4) input").value = totalMoment.toFixed(2);
}

// Bind the calculateValues function to the input change events
document.querySelectorAll(".wb1 input[type='number']").forEach(function(input) {
    input.addEventListener("input", calc_wb_part_1);
});