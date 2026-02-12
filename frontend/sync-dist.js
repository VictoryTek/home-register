#!/usr/bin/env node

/**
 * Frontend Build Sync Script
 * 
 * Copies built frontend assets from frontend/dist/ to ../static/
 * for the Rust backend to serve. This replicates the Docker build process
 * for local development without Docker.
 * 
 * Usage: node sync-dist.js
 * Or via npm: npm run sync-dist
 * 
 * Note: Converted to ES module syntax to match package.json "type": "module"
 * Changes: require() → import, added __dirname replacement for ES modules
 */

import fs from 'fs-extra';
import path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

// ES module replacement for __dirname (not available in ES modules)
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Resolve paths relative to this script's location (frontend/)
const distDir = path.join(__dirname, 'dist');
const staticDir = path.join(__dirname, '..', 'static');
const publicDir = path.join(__dirname, 'public');

/**
 * Main sync function
 * Copies dist/ contents and public/ assets to static/ directory
 */
async function syncFrontend() {
  console.log('🔄 Syncing frontend build to static directory...');
  console.log(`   Source: ${distDir}`);
  console.log(`   Destination: ${staticDir}\n`);
  
  try {
    // Validate source directory exists
    if (!fs.existsSync(distDir)) {
      console.error('❌ Error: frontend/dist/ does not exist.');
      console.error('   Please run "npm run build" first to generate build artifacts.');
      process.exit(1);
    }

    // Remove existing static directory to ensure clean state
    if (fs.existsSync(staticDir)) {
      console.log('🗑️  Removing old static directory...');
      await fs.remove(staticDir);
    }

    // Create fresh static directory
    await fs.ensureDir(staticDir);
    console.log('📁 Created static directory\n');

    // Copy all dist/ contents to static/
    console.log('📦 Copying dist/ → static/...');
    await fs.copy(distDir, staticDir, {
      overwrite: true,
      errorOnExist: false
    });
    console.log('   ✓ All dist files copied\n');

    // Copy public assets (logos, favicon) to static root
    console.log('🖼️  Copying public assets...');
    const publicAssets = [
      'logo_icon.png',
      'logo_full.png',
      'logo_icon3.png',
      'favicon.ico'
    ];

    for (const asset of publicAssets) {
      const src = path.join(publicDir, asset);
      const dest = path.join(staticDir, asset);
      
      if (fs.existsSync(src)) {
        await fs.copy(src, dest);
        console.log(`   ✓ ${asset}`);
      } else {
        console.log(`   ⚠️  ${asset} (not found in public/)`);
      }
    }

    // Copy manifest.json if it exists (fallback for older builds)
    const manifestSrc = path.join(publicDir, 'manifest.json');
    const manifestDest = path.join(staticDir, 'manifest.json');
    if (fs.existsSync(manifestSrc) && !fs.existsSync(manifestDest)) {
      await fs.copy(manifestSrc, manifestDest);
      console.log('   ✓ manifest.json\n');
    } else {
      console.log('');
    }

    // Verify critical files and provide summary
    console.log('✅ Frontend sync completed successfully!\n');
    console.log('📋 Verification - Key files:');
    
    const keyFiles = [
      'index.html',
      'sw.js',
      'manifest.webmanifest'
    ];
    
    let allFilesPresent = true;
    
    for (const file of keyFiles) {
      const filePath = path.join(staticDir, file);
      if (fs.existsSync(filePath)) {
        const stats = fs.statSync(filePath);
        const sizeKB = (stats.size / 1024).toFixed(2);
        console.log(`   ✓ ${file} (${sizeKB} KB)`);
      } else {
        console.log(`   ❌ ${file} (MISSING)`);
        allFilesPresent = false;
      }
    }

    // Check for Workbox runtime files
    const files = await fs.readdir(staticDir);
    const workboxFiles = files.filter(f => f.startsWith('workbox-') && f.endsWith('.js'));
    
    if (workboxFiles.length > 0) {
      const workboxPath = path.join(staticDir, workboxFiles[0]);
      const stats = fs.statSync(workboxPath);
      const sizeKB = (stats.size / 1024).toFixed(2);
      console.log(`   ✓ ${workboxFiles[0]} (${sizeKB} KB)`);
    } else {
      console.log('   ⚠️  No workbox-*.js files found (service worker may not work)');
      allFilesPresent = false;
    }

    // Check assets directory
    const assetsDir = path.join(staticDir, 'assets');
    if (fs.existsSync(assetsDir)) {
      const assetFiles = await fs.readdir(assetsDir);
      console.log(`   ✓ assets/ directory (${assetFiles.length} files)\n`);
    } else {
      console.log('   ⚠️  assets/ directory not found\n');
    }

    // Final status
    if (allFilesPresent) {
      console.log('🎉 All critical files present. You can now run "cargo run" to start the backend.\n');
      process.exit(0);
    } else {
      console.log('⚠️  Some files are missing. Service worker may not function correctly.');
      console.log('   Try running "npm run build" again to regenerate all files.\n');
      process.exit(1);
    }

  } catch (error) {
    console.error('\n❌ Sync failed with error:');
    console.error(`   ${error.message}`);
    
    if (error.code === 'EACCES') {
      console.error('\n   Permission denied. Try running with appropriate permissions.');
    } else if (error.code === 'ENOSPC') {
      console.error('\n   No space left on device. Free up some disk space and try again.');
    }
    
    process.exit(1);
  }
}

// Execute sync
syncFrontend();
