export const getFavicon = async (websiteUrl: string | undefined, size: number = 64) => {
    // try to get the favicon from google, if it fails, return an empty string
    const faviconUrl = `https://www.google.com/s2/favicons?domain=${websiteUrl}&sz=${size}`;
    const response = await fetch(faviconUrl);
    if(response.ok){
        return faviconUrl;
    } else return "";
};