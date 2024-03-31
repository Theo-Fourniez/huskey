<script setup lang="ts">

const pickRandomFaIcon = () => {
    const faIcons = [
        "fa-facebook",
        "fa-twitter",
        "fa-instagram",
        "fa-tiktok",
        "fa-linkedin",
        "fa-github",
        "fa-discord",
        "fa-youtube",
        "fa-wordpress",
        "fa-slack",
        "fa-figma",
        "fa-apple",
        "fa-google",
        "fa-stripe",
        "fa-algolia",
        "fa-docker",
        "fa-windows",
        "fa-paypal",
        "fa-stack-overflow",
        "fa-kickstarter",
        "fa-dribbble",
        "fa-dropbox",
        "fa-squarespace",
        "fa-shopify",
        "fa-medium",
        "fa-codepen",
        "fa-cloudflare",
        "fa-airbnb",
        "fa-vimeo",
        "fa-whatsapp",
        "fa-intercom",
        "fa-usps",
        "fa-wix",
        "fa-line",
        "fa-behance",
        "fa-openid",
        "fa-product-hunt",
        "fa-pagelines",
        "fa-teamspeak",
        "fa-telegram",
        "fa-pinterest",
        "fa-dashcube",
        "fa-ideal",
        "fa-salesforce",
        "fa-free-code-camp",
        "fa-soundcloud",
        "fa-square-twitter",
        "fa-cc-visa",
        "fa-goodreads-g",
        "fa-react",
        "fa-wikipedia-w",
        "fa-square-js",
        "fa-square-pinterest",
        "fa-skype",
        "fa-linux",
        "fa-node",
        "fa-rebel",
        "fa-etsy",
        "fa-discourse",
        "fa-amazon",
        "fa-glide-g",
        "fa-gitlab",
        "fa-spotify",
        "fa-think-peaks",
        "fa-microsoft",
        "fa-elementor",
        "fa-pied-piper",
        "fa-square-youtube",
        "fa-cc-mastercard",
        "fa-facebook-messenger",
        "fa-atlassian",
        "fa-playstation",
    ]; 
    return faIcons[Math.floor(Math.random() * faIcons.length)];
};
</script>

<template>
    <div style="height: 48px; width:48px; display: flex; align-items: center; background-color: var(--color-dark-accent); justify-content: center;  border-radius: 5px;">
        <i :class="'fa-brands ' + pickRandomFaIcon()" style="font-size: 35px; color:#ffffff;"></i>
    </div>
</template>

<style scoped>
    img {
        width: 128px;
        height: 128px;
    }
</style>
