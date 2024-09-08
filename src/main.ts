import { invoke } from "@tauri-apps/api/tauri";

let num1: HTMLInputElement | null;
let num2: HTMLInputElement | null;
let input: HTMLInputElement | null;
let btn: HTMLInputElement | null;
let added: HTMLElement | null;
let output: HTMLElement | null;

async function adder() {
    if (num1 && num2 && added) {
        var asnum1: number = +num1.value;
        var asnum2: number = +num2.value;
        added.textContent = await invoke("adder", {
            num1: asnum1,
            num2: asnum2
        });
    }
}

async function calc() {
    console.log("calc");
    if (input && output) {
        console.log("input: ", input.value);
        let list: string[] = await invoke("calc", {
            input: input.value
        });
        if (list.length == 1) {
            output.textContent = list[0];
        }
        output.innerHTML = list.join("<br>");
    }
}


window.addEventListener("DOMContentLoaded", () => {
    num1 = document.querySelector("#num1");
    num2 = document.querySelector("#num2");
    added = document.querySelector("#added");
    input = document.querySelector("#input");
    btn = document.querySelector("#input-btn");
    output = document.querySelector("#output");

    document.querySelector("#adder-form")?.addEventListener("submit", (e) => {
        e.preventDefault();
        adder();
    });
    btn?.addEventListener("click", (e) => {
        e.preventDefault();
        calc();
    });

});

