from flask import Flask, render_template, request, session, redirect

app = Flask(__name__)

app.secret_key = b'_5#y2L"F4Q8z\n\xec]/'

@app.route('/auth')
def auth():
    return render_template("auth.html")

@app.route('/login', methods = ['POST'])
def login():
    if request.method == 'POST':
        login = (request.form['login'])
        password = (request.form['password'])
        print(password)
        return redirect("/")

@app.route('/logout')
def logout():
    session.clear()
    return redirect("/auth")


@app.route('/')
def index():
    return render_template("main.html")




if(__name__) == "__main__":
    app.run(debug=True, port=5000)



