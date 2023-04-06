import * as $ from 'jquery';

let isMenuOpen = true;

let navbutton = document.getElementById("navbutton")!
let navMenu = document.getElementById("menu")!

navbutton.onclick = (e) => {
    navMenu.classList.toggle("collapse")
}

if ($(window).width() < 1024) {
    navMenu.classList.add("collapse")
    isMenuOpen = false;
}
