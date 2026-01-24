# Script PowerShell pour créer une icône simple
Add-Type -AssemblyName System.Drawing

# Créer une image 1024x1024
$bitmap = New-Object System.Drawing.Bitmap(1024, 1024)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)

# Fond bleu
$blueBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(100, 108, 255))
$graphics.FillRectangle($blueBrush, 0, 0, 1024, 1024)

# Texte "V"
$font = New-Object System.Drawing.Font("Arial", 400, [System.Drawing.FontStyle]::Bold)
$whiteBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$stringFormat = New-Object System.Drawing.StringFormat
$stringFormat.Alignment = [System.Drawing.StringAlignment]::Center
$stringFormat.LineAlignment = [System.Drawing.StringAlignment]::Center
$graphics.DrawString("V", $font, $whiteBrush, 512, 512, $stringFormat)

# Sauvegarder
$bitmap.Save("app-icon.png", [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Icône créée : app-icon.png"

# Nettoyage
$graphics.Dispose()
$bitmap.Dispose()
$blueBrush.Dispose()
$whiteBrush.Dispose()
$font.Dispose()
