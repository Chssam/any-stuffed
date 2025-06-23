import init, { run_in_background } from './refresher.js';

console.log("Background script running");

async function output_date() {
    const the_date = new Date;
    console.log("Started at ", the_date);
}

output_date();

async function background_runner() {
    try {
        await init();
        console.log("Successful Init background");
        run_in_background();
    } catch (error) {
        console.error('Failed to run background:', error);
    }
}

background_runner();
