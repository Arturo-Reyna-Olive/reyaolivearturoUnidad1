import sys
import smtplib
from email.message import EmailMessage

# Lee argumentos: email y link
if len(sys.argv) != 3:
    print("Uso: python send_mail.py <correo> <link>")
    sys.exit(1)

to_email = sys.argv[1]
reset_link = sys.argv[2]

# tu cuenta 
SMTP_EMAIL = "arturoreyna694@gmail.com"
SMTP_PASS = "ywjplzbcfdxblpqi"

msg = EmailMessage()
msg["Subject"] = "🔐 Recupera tu contraseña - Genomatrix"
msg["From"] = SMTP_EMAIL
msg["To"] = to_email
msg.set_content(f"""Hola,\n\nPara recuperar tu contraseña haz clic en el siguiente enlace:\n{reset_link}\n\nEste link es temporal.""")

try:
    with smtplib.SMTP("smtp.gmail.com", 587) as server:
        server.starttls()
        server.login(SMTP_EMAIL, SMTP_PASS)
        server.send_message(msg)
    print("Correo enviado")
except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)
