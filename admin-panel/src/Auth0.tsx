import { Auth0Provider } from "@auth0/auth0-react";
import { useEffect, useState } from "react";

interface Auth0Config {
    domain: string;
    client_id: string;
    audience: string;
}

const DynamicAuth0 = ({ children }: { children: React.ReactNode }) => {
    const [config, setConfig] = useState<Auth0Config | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetch('/api/auth0-config')  // Adjust this endpoint to match your API
            .then(response => {
                if (!response.ok) {
                    throw new Error('Failed to fetch Auth0 configuration');
                }
                return response.json();
            })
            .then(data => setConfig(data))
            .catch(err => setError(err.message));
    }, []);

    if (error) {
        return <div>Error loading authentication: {error}</div>;
    }

    if (!config) {
        return <div>Loading authentication configuration...</div>;
    }

    console.log(config);

    return (
        <Auth0Provider
            domain={config.domain}
            clientId={config.client_id}
            audience={config.audience}
            cacheLocation="localstorage"
            redirectUri={window.location.origin}
        >
            {children}
        </Auth0Provider>
    );
}

export default DynamicAuth0;