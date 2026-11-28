package com.inta.mobile;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.security.GeneralSecurityException;

/** Interoperability fixture using the production pairing and authenticated reader across stdin/stdout. */
public final class DiagnosticPeer {
    private DiagnosticPeer() {}
    public static void main(String[] args) throws Exception {
        BufferedReader input = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
        DiagnosticHandoff session=null;
        for (String line; (line=input.readLine())!=null; ) {
            long wall=System.currentTimeMillis()/1000L, elapsed=System.nanoTime()/1_000_000L;
            if (line.equals("new")) {
                if(session!=null)session.revoke();
                session=new DiagnosticHandoff(wall,elapsed);System.out.println(session.code(wall,elapsed));
            } else if(line.equals("revoke") && session!=null) {
                session.revoke();System.out.println("revoked");
            } else if(line.equals("quit")) { if(session!=null)session.revoke(); break; }
            else if(session!=null && (line.startsWith("read ")||line.startsWith("expired ")||line.startsWith("elapsed "))) {
                try {
                    byte[] bytes=Files.readAllBytes(Paths.get(line.substring(line.indexOf(' ')+1)));
                    DiagnosticHandoff.Snapshot snap=session.read(bytes,line.startsWith("expired ")?wall+601:wall,
                        line.startsWith("elapsed ")?elapsed+601000:elapsed);
                    System.out.println("read "+snap.captured+" "+snap.total+" "+snap.available+" "+snap.load+" "+snap.ac+" "+snap.battery);
                } catch(GeneralSecurityException denied) {System.out.println("refused");}
            } else throw new IllegalArgumentException("Unknown fixture request");
        }
    }
}
