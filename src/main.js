const { appWindow } = window.__TAURI__.window;
const images = document.getElementsByTagName('img');


document.getElementById('titlebar-minimize').addEventListener('click', () => appWindow.minimize());
document.getElementById('titlebar-close').addEventListener('click', () => appWindow.close());


for (let i = 0; i < images.length; i++) {
	images[i].addEventListener('mousedown', function (e) {
		e.preventDefault();
	});
}

document.addEventListener('contextmenu', function (e) {
	e.preventDefault();
});


// Download Chromium
const downloadChromiumBtn = document.getElementById('downloadChromium');

async function downloadChromium() {
	try {
		const downloadChromiumBtn = document.getElementById('downloadChromium');
		downloadChromiumBtn.innerHTML = `
			<img src="assets/images/ui/loader.svg" class="spin">
			Installing Chromium Browser
		`;
		downloadChromiumBtn.title = 'Downloading Chromium Browser';
		downloadChromiumBtn.disabled = true;
		new Notification('Downloading Chromium', {
			body: 'Chromium is downloading. This may take a few minutes.',
			sound: 'Default'
		});
		await window.__TAURI__.invoke('download_chromium', {
			url: 'https://download-chromium.appspot.com/dl/Win_x64?type=snapshots',
			destination: 'C:\\Chromium Project'
		});
		new Notification('Installation Complete', {
			body: 'Chromium has been successfully installed. Enjoy!',
			sound: 'Default'
		});
		downloadChromiumBtn.innerHTML = `
			<img src="assets/images/ui/download.svg">
			Download Chromium Browser
		`;
		displayChromiumVersion();
	} catch (error) {
		console.error('Failed to download Chromium:', error);
		new Notification('Download Failed', {
			body: 'Failed to download Chromium. Please try again later',
			sound: 'Default'
		});
	}
}

downloadChromiumBtn.addEventListener('click', downloadChromium);


// Open Chromium
const openChromiumBtn = document.getElementById('openChromium');

async function openChromium() {
	try {
		await window.__TAURI__.invoke('open_chromium', {
			destination: 'C:\\Chromium Project\\Chromium\\chrome.exe'
		});
	} catch (error) {
		console.error('Failed to open Chromium:', error);
		new Notification('Open Failed', {
			body: 'Failed to open Chromium. Please try again later',
			sound: 'Default'
		});
	}
}

openChromiumBtn.addEventListener('click', openChromium);


// Check for Updates
const checkUpdatesBtn = document.getElementById('checkUpdates');

async function checkUpdates() {
	checkUpdatesBtn.innerHTML = `
		<img src="assets/images/ui/loader.svg" class="spin">
		Checking for Updates
	`;
	checkUpdatesBtn.disabled = true;
	checkUpdatesBtn.title = 'You have the latest version of Chromium installed';

	try {
		const response = await fetch('https://chromium.woolyss.com/api/?os=windows&bit=64&out=json');
		const data = await response.json();
		const currentVersion = document.getElementById('chromiumVersion').innerText;
		if (currentVersion === data.chromium.windows.version || currentVersion > data.chromium.windows.version) {
			new Notification('Chromium is Up to Date', {
				body: 'You already have the latest version of Chromium installed. Enjoy!',
				sound: 'Default'
			});
			checkUpdatesBtn.innerHTML = `
				<img src="assets/images/ui/check.svg">
				Up to Date
			`;
		} else {
			new Notification('Update Available', {
				body: `Chromium ${data.chromium.windows.version} is available. Download now!`,
				sound: 'Default'
			});
			checkUpdatesBtn.innerHTML = `
				<img src="assets/images/ui/download.svg">
				Update Available
			`;
			downloadChromiumBtn.disabled = false;
			downloadChromiumBtn.title = 'Download the latest version of Chromium Browser';
		}
	} catch (error) {
		console.error('Error fetching Chromium version:', error);
		new Notification('Update Check Failed', {
			body: 'Failed to check for Chromium updates. Please try again later.',
			sound: 'Default'
		});
	} finally {
		setTimeout(() => {
			checkUpdatesBtn.disabled = false;
			checkUpdatesBtn.title = 'Check updates for your current Chromium installation';
			checkUpdatesBtn.innerHTML = `
				<img src="assets/images/ui/refresh.svg">
				Check for Updates
			`;
		}, 300000); // 300000 milliseconds = 5 minutes
	}
}

checkUpdatesBtn.addEventListener('click', checkUpdates);



// Uninstall Chromium
const uninstallChromiumBtn = document.getElementById('uninstallChromium');

async function uninstallChromium() {
	try {
		const chromiumPath = 'C:\\Chromium Project';
		await window.__TAURI__.invoke('uninstall_chromium', { chromiumPath: chromiumPath });
		new Notification('Uninstall Complete', {
			body: 'Chromium has been successfully uninstalled.',
			sound: 'Default'
		});
		downloadChromiumBtn.innerHTML = `
			<img src="assets/images/ui/download.svg">
			Download Chromium Browser
		`;
		downloadChromiumBtn.disabled = false;
		downloadChromiumBtn.title = 'Chromium Browser will be installed to "C:\\Chromium Project\\Chromium"';
		checkUpdatesBtn.innerHTML = `
			<img src="assets/images/ui/refresh.svg">
			Check for Updates
		`;
		checkUpdatesBtn.title = 'Check updates for your current Chromium installation';
		displayChromiumVersion();
	} catch (error) {
		console.error('Failed to uninstall Chromium:', error);
	}
}

uninstallChromiumBtn.addEventListener('click', uninstallChromium);




// Check Version
async function displayChromiumVersion() {
	try {
		const chromiumPath = 'C:\\Chromium Project\\Chromium';
		const version = await window.__TAURI__.invoke('get_chromium_version', { chromiumPath: chromiumPath });
		document.getElementById('chromiumVersion').innerText = `${version}`;
		document.getElementById('checkUpdates').disabled = false;
		document.getElementById('checkUpdates').title = 'Check updates for your current Chromium installation';
		document.getElementById('openChromium').disabled = false;
		document.getElementById('openChromium').title = 'Open Chromium Browser';
		document.getElementById('uninstallChromium').disabled = false;
		document.getElementById('uninstallChromium').title = 'Uninstall Chromium Browser';
		document.getElementById('downloadChromium').disabled = true;
		document.getElementById('downloadChromium').title = 'Chromium is already installed in "C:\\Chromium Project\\Chromium"';
		document.getElementById('downloadChromium').innerHTML = `
			<img src="assets/images/ui/check.svg">
			Chromium Installed
		`;
	} catch (error) {
		console.error('Failed to get Chromium version:', error);
		if (error === 'Provided path is not a directory') {
			document.getElementById('chromiumVersion').innerText = `Chromium is not installed`;
			document.getElementById('openChromium').disabled = true;
			document.getElementById('openChromium').title = 'Chromium is not installed';
			document.getElementById('checkUpdates').disabled = true;
			document.getElementById('checkUpdates').title = 'Chromium is not installed';
			document.getElementById('uninstallChromium').disabled = true;
			document.getElementById('uninstallChromium').title = 'Chromium is not installed';
			document.getElementById('downloadChromium').disabled = false;
			document.getElementById('downloadChromium').title = title = 'Chromium Browser will be installed to "C:\\Chromium Project\\Chromium"';
		} else {
			document.getElementById('chromiumVersion').innerText = `Failed to get Chromium version`;
			document.getElementById('openChromium').disabled = true;
			document.getElementById('openChromium').title = 'Failed to get Chromium version';
			document.getElementById('checkUpdates').disabled = true;
			document.getElementById('checkUpdates').title = 'Failed to get Chromium version';
			document.getElementById('uninstallChromium').disabled = true;
			document.getElementById('uninstallChromium').title = 'Failed to get Chromium version';
			document.getElementById('downloadChromium').disabled = false;
			document.getElementById('downloadChromium').title = title = 'Chromium Browser will be installed to "C:\\Chromium Project\\Chromium"';
		}
	}
}

displayChromiumVersion();



window.addEventListener('keydown', function (event) {
	if (event.keyCode === 116 || (event.ctrlKey && event.keyCode === 82)) {
		event.preventDefault();
	}
});