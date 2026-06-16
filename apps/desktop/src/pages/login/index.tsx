import { useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import { Zap } from "lucide-react";
import { useAuth } from "@/hooks/use-auth";
import { UserAuthForm } from "@/components/user-auth-form";

export default function LoginPage() {
  const { isAuthenticated, isLoading } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (!isLoading && isAuthenticated) {
      navigate("/", { replace: true });
    }
  }, [isAuthenticated, isLoading, navigate]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen bg-background">
        <div className="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background flex relative">
      {/* Logo - Fixed position matching website header exactly */}
      <nav className="fixed top-0 left-0 right-0 z-50 w-full pointer-events-none">
        <div className="relative py-3 xl:py-4 px-4 sm:px-4 md:px-4 lg:px-4 xl:px-6 2xl:px-8 flex items-center">
          <Link
            to="/"
            className="flex items-center gap-2 hover:opacity-80 active:opacity-80 transition-opacity duration-200 pointer-events-auto"
          >
            <div className="w-6 h-6">
              <Zap className="w-full h-full text-foreground" />
            </div>
          </Link>
        </div>
      </nav>

      {/* Login Form */}
      <div className="w-full flex-1 flex flex-col items-center p-8 lg:p-12 pb-2">
        <div className="w-full max-w-md flex flex-col flex-1">
          <div className="space-y-8 flex-1 flex flex-col justify-center">
            {/* Header */}
            <div className="text-center space-y-2">
              <h1 className="text-lg mb-4 font-serif">
                Welcome to Orbis School
              </h1>
              <p className="font-sans text-sm text-muted-foreground">
                Sign in or create an account
              </p>
            </div>

            {/* Sign In Option - Password Login is standard for desktop */}
            <div className="space-y-3 flex flex-col items-center justify-center w-full">
              <UserAuthForm className="w-full" />
            </div>
          </div>

          {/* Terms and Privacy Policy - Bottom aligned */}
          <div className="text-center mt-auto">
            <p className="font-sans text-xs text-muted-foreground">
              By signing in you agree to our{" "}
              <Link
                to="https://orbis.ai/terms"
                className="text-muted-foreground hover:text-primary transition-colors underline"
              >
                Terms of service
              </Link>{" "}
              &{" "}
              <Link
                to="https://orbis.ai/policy"
                className="text-muted-foreground hover:text-primary transition-colors underline"
              >
                Privacy policy
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
