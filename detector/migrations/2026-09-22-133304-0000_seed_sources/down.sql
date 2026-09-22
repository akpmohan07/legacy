DELETE FROM sources WHERE name IN (
    'application', 'homebrew-cellar', 'homebrew-cask', 'cargo', 'npm-global', 'mac-app-store'
);
