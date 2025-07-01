import java.util.*;


public record Account(
        String id,
        Optional<String> name,
        boolean suspended
) {

    public Account {
        Objects.requireNonNull(id);
        Objects.requireNonNull(name); // prevent null Optional
    }

    public static AccountBuilder start(String id) {
        return AccountBuilder.builder(id); // required value
    }

}
