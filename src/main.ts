import { invoke } from "@tauri-apps/api/tauri";

let num1: HTMLInputElement | null;
let num2: HTMLInputElement | null;
let added: HTMLElement | null;

async function adder() {
    if (num1 && num2 && added) {
        var asnum1: number = +num1.value;
        var asnum2: number = +num2.value;
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        added.textContent = await invoke("adder", {
            num1: asnum1,
            num2: asnum2
        });
    }
}

window.addEventListener("DOMContentLoaded", () => {
    num1 = document.querySelector("#num1");
    num2 = document.querySelector("#num2");
    added = document.querySelector("#added");
    document.querySelector("#adder-form")?.addEventListener("submit", (e) => {
        e.preventDefault();
        adder();
    });
});

