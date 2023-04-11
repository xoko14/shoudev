import * as $ from 'jquery';

let isMenuOpen = true;

let navbutton = document.getElementById("navbutton")!
let navMenu = document.getElementById("menu")!

navbutton.onclick = (e) => {
    navMenu.classList.toggle("hidden")
}

if ($(window).width() < 1024) {
    navMenu.classList.add("hidden")
    isMenuOpen = false;
}
