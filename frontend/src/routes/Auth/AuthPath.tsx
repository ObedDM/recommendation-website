import { useState } from "react"
import AuthBackground from "../../components/Other/Background/AuthBackground";
import LoginPanel from "../../components/Auth/Login/LoginPanel";
import SignupPanel from "../../components/Auth/Signup/SignupPanel";
import LoginNavBar from "../../components/Auth/Login/LoginNavBar";

export default function AuthPath() {
  const navItems = ['PRICING', 'CONTACT', 'TERMS OF USE', 'API', 'COOKIES'];
  const [showLogin, setShowLogin] = useState(true);
  const [fade, setFade] = useState(false);

  const togglePanel = () => {
    setFade(true);
    setTimeout(() => {
      setShowLogin(!showLogin);
      setFade(false);
    }, 200);
  };

  return (
    <>
      <main className="relative w-full h-screen">
        <AuthBackground />

        <div className={`absolute top-1/2 right-1/8 transform -translate-y-1/2 transition-all duration-200 ease-in-out ${fade ? 'opacity-0 translate-x-[100px]' : 'opacity-100 translate-x-0'}`}>
          {showLogin ? (
            <LoginPanel
              buttonLogIn= {["Log in", "custom-login-gradient"]}
              buttonCreateAccount={["Create account", "custom-createAcc-gradient"]}
              onSwitch={togglePanel}/>
          ) : (
            <SignupPanel
              buttonSignUp= {["Sign up", "custom-login-gradient"]}
              onSwitch={togglePanel}/>
          )}
        </div>
      </main>

      <footer>
        <div className="fixed bottom-0 w-full">
          {<LoginNavBar navItems={navItems} />}
        </div>
      </footer>
    </>
  )
}