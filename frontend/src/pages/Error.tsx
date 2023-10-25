import { useRouteError } from "react-router-dom";

function ErrorPage() {
  const error: unknown = useRouteError();

  return (
    <div>
      <h1>Oops!</h1>
      <p>The page you are looking for does not exit.</p>
      <p>
        <i>
          {(error as Error)?.message ||
            (error as { statusText?: string })?.statusText}
        </i>
      </p>
    </div>
  );
}

export default ErrorPage;
