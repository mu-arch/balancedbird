//GLOBALS

let maxToWt = 0;
let maxLdgWt = 0;
const loader ="<svg style='position:relative; top:2px;' xmlns=\"http://www.w3.org/2000/svg\" width=\"1em\" height=\"1em\" viewBox=\"0 0 24 24\"><rect width=\"7.33\" height=\"7.33\" x=\"1\" y=\"1\" fill=\"currentColor\"><animate id=\"svgSpinnersBlocksWave0\" attributeName=\"x\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"y\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"width\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"0;svgSpinnersBlocksWave1.end+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"8.33\" y=\"1\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"1\" y=\"8.33\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.1s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"15.66\" y=\"1\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"8.33\" y=\"8.33\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"1\" y=\"15.66\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"1;4;1\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.2s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"15.66\" y=\"8.33\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"8.33\" y=\"15.66\" fill=\"currentColor\"><animate attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"8.33;11.33;8.33\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.3s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect><rect width=\"7.33\" height=\"7.33\" x=\"15.66\" y=\"15.66\" fill=\"currentColor\"><animate id=\"svgSpinnersBlocksWave1\" attributeName=\"x\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"y\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"15.66;18.66;15.66\"/><animate attributeName=\"width\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/><animate attributeName=\"height\" begin=\"svgSpinnersBlocksWave0.begin+0.4s\" dur=\"0.6s\" values=\"7.33;1.33;7.33\"/></rect></svg>";



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

/*
document.getElementById("clear").addEventListener("click", function () {
    document.querySelectorAll("input").forEach(function (input) {
        input.value = "";
    });
});

 */


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

function getRandomDelay() {
    return Math.floor(Math.random() * (2000 - 100 + 1)) + 500;
}
/*
function submitAirportId() {
    const airportId = document.getElementById('weather-input').value.trim().toUpperCase();

    document.getElementById('weather-table').style.display = "none";
    document.getElementById('weather-text').style.display = "block";

    if (airportId.length === 0) {
        document.getElementById('weather-text').innerHTML = "Airport ICAO identifier cannot be empty.";

        return
    }

    document.getElementById('weather-text').innerHTML = loader + " Please wait. Contacting FAA systems...";

    delay(400, function() {
        document.getElementById('weather-text').innerHTML = loader + " Web scraping and compiling sources...";

        delay(300, function() {
            document.getElementById('weather-text').innerHTML = loader + " Acquiring latest weather observation from aviationweather.gov...";

            delay(1000, function() {

                var http = new XMLHttpRequest();
                var url = 'http://127.0.0.1:3000/weather/' + airportId;
                http.open('GET', url, true);

                http.onreadystatechange = function() {
                    if (http.readyState == 4) {
                        if (http.status == 200) {
                            var response = JSON.parse(http.responseText);
                            updateWeatherTable(response);

                            document.getElementById('weather-text').innerHTML = response.raw_ob || 'N/A';
                            document.getElementById('weather-table').style.display = "block";
                        } else {
                            let metar_error = "Something went wrong. The weather system may be down or the server is not responding.";
                            document.getElementById('weather-text').innerHTML = metar_error;
                        }
                    }
                };
                http.send();
            });
        });
    });

    // Utility function to create delays
    function delay(duration, callback) {
        setTimeout(callback, duration);
    }
}

 */

// Utility function to create delays
function delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function submitAirportId() {
    const airportId = document.getElementById('weather-input').value.trim().toUpperCase();

    document.getElementById('weather-table').style.display = "none";
    document.getElementById('weather-text').style.display = "block";

    if (airportId.length === 0) {
        document.getElementById('weather-text').innerHTML = "Airport ICAO identifier cannot be empty.";
        return;
    }
    document.getElementById('weather-text').innerHTML = loader + " Downloading and processing FAA.gov airport database...";


    try {

        // Initiate the web request immediately using fetch
        const response = await fetch('http://127.0.0.1:3000/weather/' + airportId);

        if (!response.ok) {
            throw new Error(await response.text());
        }

        const data = await response.json();
        updateWeatherTable(data);


        document.getElementById('weather-text').innerHTML = loader + " Acquiring latest observation from aviationweather.gov...";


        // Perform the second delay of 300ms
        await delay(1200);

        document.getElementById('weather-text').innerHTML = data.raw_ob || 'N/A';
        document.getElementById('weather-table').style.display = "block";

    } catch (error) {
        // Handle errors gracefully
        document.getElementById('weather-text').innerHTML = error.message;
    }
}

// New function to update the weather table with the JSON response
function updateWeatherTable(response) {
    // Select the weather table
    var table = document.querySelector('#weather-table table.std');
    var rows = table.rows;





    // Row 1: Airport and Best Runway
    rows[0].cells[1].innerHTML = response.name || 'N/A';
    rows[0].cells[3].innerHTML = response.best_runway
        ? `${response.best_runway} (${response.best_runway_heading_magnetic}°)`
        : 'N/A';


    // Row 2: Observation and Crosswind
    rows[1].cells[1].innerHTML = zuluToLocalReadableTime(response.obs_time) + " Local" || 'Unknown';




    var surfaceWind = (response.wdir && response.wspd)
        ? `${response.wdir}° ${response.wspd}kt` + (response.wgst ? ` G${response.wgst}` : '')
        : 'Not available';
    rows[2].cells[1].innerHTML = surfaceWind;


    rows[1].cells[3].innerHTML = response.xwind
        ? response.xwind.toFixed(1) + 'kt' + (response.gxwind ? ' G' + response.gxwind.toFixed(1) : '')
        : 'Not available';

// Update Headwind Cell
    rows[2].cells[3].innerHTML = response.hwind
        ? response.hwind.toFixed(1) + 'kt' + (response.ghwind ? ' G' + response.ghwind.toFixed(1) : '')
        : 'Not available';

    // Row 4: Vis / Weather and Field Elevation
    var visibility = parseVisibility(response.raw_ob);
    var weather = parseWeather(response.raw_ob);
    rows[3].cells[1].innerHTML = `${visibility} / ${weather}`;
    rows[3].cells[3].innerHTML = response.field_elevation ? `${response.field_elevation}ft` : 'Not available';

    // Row 5: Temp / Dew Pt. and Pressure Altitude
    rows[4].cells[1].innerHTML = (response.temp !== undefined && response.dewp !== undefined) ? `${response.temp}° / ${response.dewp}°` : 'Not available';

    rows[4].cells[3].innerHTML = response.pressure_altitude ? `${response.pressure_altitude}ft` : 'Not available';

    // Row 6: Altimeter and Density Altitude
    rows[5].cells[1].innerHTML = response.altimeter ? `${response.altimeter} inHg` : 'Not available';
    rows[5].cells[3].innerHTML = response.density_altitude ? `${response.density_altitude}ft` : 'Not available';
}

// Event listener for the "Load Airport Information" button
document.querySelector('button[onclick="submitAirportId()"]').addEventListener('click', submitAirportId);

// Optional: Handle Enter key press in the input field
document.getElementById('weather-input').addEventListener('keydown', function(event) {
    if (event.key === 'Enter') {
        submitAirportId();
    }
});

// Helper functions to parse visibility and weather from raw_ob
function parseVisibility(ob) {
    var match = ob.match(/(\d+)?\s*SM/);
    return match ? match[0] : 'N/A';
}

function parseWeather(ob) {
    var match = ob.match(/SM\s+(\w+)/);
    return match ? match[1] : 'N/A';
}

const zuluToLocalReadableTime = zulu => {
    // Extract components from the Zulu time format
    const day = parseInt(zulu.substring(0, 2), 10);
    const hour = parseInt(zulu.substring(2, 4), 10);
    const minute = parseInt(zulu.substring(4, 6), 10);

    // Create a Date object in UTC using the current year and month for simplicity
    const date = new Date(Date.UTC(new Date().getUTCFullYear(), new Date().getUTCMonth(), day, hour, minute));

    // Convert to local time string
    return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: 'numeric', hour12: true });
};
