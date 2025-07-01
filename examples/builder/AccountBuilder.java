import java.util.*;

public class AccountBuilder {

    private final String id;
    private Optional<String> name;
    private boolean suspended;

    private AccountBuilder(
        String id,
        Optional<String> name,
        boolean suspended
    ) {
        this.id = Objects.requireNonNull(id); // prevent missing default value
        this.name = Objects.requireNonNull(name); // prevent empty optional
        this.suspended = suspended;
    }

    public static AccountBuilder builder(String id) {
        return new AccountBuilder(
                id,
                Optional.empty(),
                false
        );
    }

    public Account build() {
        return new Account(
            this.id,
            this.name,
            this.suspended
        );
    }


    public AccountBuilder company(String name) {
        var validName = Objects.requireNonNull(name); // when this is used, the parameter is no longer optional
        this.name = Optional.of(validName);
        return this;
    }

    public AccountBuilder suspended() { // no need for the inverse
        this.suspended = true;
        return this;
    }

}
