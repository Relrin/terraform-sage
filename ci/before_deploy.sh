# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          artefact = terraform-sage.exe
          stage= \

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    case $TARGET in
        x86_64-pc-windows-gnu)
            artefact=terraform-sage.exe
            ;;
        x86_64-unknown-linux-gnu)    
            artefact=terraform-sage
            ;;
        x86_64-unknown-linux-musl)
            artefact=terraform-sage
            ;;
        x86_64-apple-darwin)
            artefact=terraform-sage
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin $artefact --target $TARGET --release -- -C lto
    cp target/$TARGET/release/$artefact $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
