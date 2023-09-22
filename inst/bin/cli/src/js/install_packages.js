export default async function installPackages(webR) {
  await webR.evalRVoid(`webr::install("shiny", repos="${window.location.href}/repo/")`);
}
