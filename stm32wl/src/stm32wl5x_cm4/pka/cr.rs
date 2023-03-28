#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - PKA enable."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "PKA enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable PKA"]
    Disabled = 0,
    #[doc = "1: Enable PKA"]
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
#[doc = "Field `EN` writer - PKA enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disable PKA"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "Enable PKA"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
#[doc = "Field `START` reader - start the operation"]
pub type START_R = crate::BitReader<STARTW_A>;
#[doc = "start the operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW_A {
    #[doc = "1: Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    Start = 1,
}
impl From<STARTW_A> for bool {
    #[inline(always)]
    fn from(variant: STARTW_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTW_A> {
        match self.bits {
            true => Some(STARTW_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTW_A::Start
    }
}
#[doc = "Field `START` writer - start the operation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, STARTW_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW_A::Start)
    }
}
#[doc = "Field `MODE` reader - PKA operation code"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "PKA operation code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Montgomery parameter computation then modular exponentiation"]
    MontgomeryCompExp = 0,
    #[doc = "1: Montgomery parameter computation only"]
    MontgomeryComp = 1,
    #[doc = "2: Modular exponentiation only (Montgomery parameter must be loaded first)"]
    MontgomeryExp = 2,
    #[doc = "7: RSA CRT exponentiation"]
    Rsa = 7,
    #[doc = "8: Modular inversion"]
    ModularInv = 8,
    #[doc = "9: Arithmetic addition"]
    ArithmeticAdd = 9,
    #[doc = "10: Arithmetic subtraction"]
    ArithmeticSub = 10,
    #[doc = "11: Arithmetic multiplication"]
    ArithmeticMul = 11,
    #[doc = "12: Arithmetic comparison"]
    ArithmeticComp = 12,
    #[doc = "13: Modular reduction"]
    ModularRed = 13,
    #[doc = "14: Modular addition"]
    ModularAdd = 14,
    #[doc = "15: Modular subtraction"]
    ModularSub = 15,
    #[doc = "16: Montgomery multiplication"]
    ModularMul = 16,
    #[doc = "32: Montgomery parameter computation then ECC scalar multiplication"]
    MontgomeryCompScalar = 32,
    #[doc = "34: ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    MontgomeryScalar = 34,
    #[doc = "36: ECDSA sign"]
    Ecdsasign = 36,
    #[doc = "38: ECDSA verification"]
    Ecdsaverif = 38,
    #[doc = "40: Point on elliptic curve Fp check"]
    Elliptic = 40,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::MontgomeryCompExp),
            1 => Some(MODE_A::MontgomeryComp),
            2 => Some(MODE_A::MontgomeryExp),
            7 => Some(MODE_A::Rsa),
            8 => Some(MODE_A::ModularInv),
            9 => Some(MODE_A::ArithmeticAdd),
            10 => Some(MODE_A::ArithmeticSub),
            11 => Some(MODE_A::ArithmeticMul),
            12 => Some(MODE_A::ArithmeticComp),
            13 => Some(MODE_A::ModularRed),
            14 => Some(MODE_A::ModularAdd),
            15 => Some(MODE_A::ModularSub),
            16 => Some(MODE_A::ModularMul),
            32 => Some(MODE_A::MontgomeryCompScalar),
            34 => Some(MODE_A::MontgomeryScalar),
            36 => Some(MODE_A::Ecdsasign),
            38 => Some(MODE_A::Ecdsaverif),
            40 => Some(MODE_A::Elliptic),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MontgomeryCompExp`"]
    #[inline(always)]
    pub fn is_montgomery_comp_exp(&self) -> bool {
        *self == MODE_A::MontgomeryCompExp
    }
    #[doc = "Checks if the value of the field is `MontgomeryComp`"]
    #[inline(always)]
    pub fn is_montgomery_comp(&self) -> bool {
        *self == MODE_A::MontgomeryComp
    }
    #[doc = "Checks if the value of the field is `MontgomeryExp`"]
    #[inline(always)]
    pub fn is_montgomery_exp(&self) -> bool {
        *self == MODE_A::MontgomeryExp
    }
    #[doc = "Checks if the value of the field is `Rsa`"]
    #[inline(always)]
    pub fn is_rsa(&self) -> bool {
        *self == MODE_A::Rsa
    }
    #[doc = "Checks if the value of the field is `ModularInv`"]
    #[inline(always)]
    pub fn is_modular_inv(&self) -> bool {
        *self == MODE_A::ModularInv
    }
    #[doc = "Checks if the value of the field is `ArithmeticAdd`"]
    #[inline(always)]
    pub fn is_arithmetic_add(&self) -> bool {
        *self == MODE_A::ArithmeticAdd
    }
    #[doc = "Checks if the value of the field is `ArithmeticSub`"]
    #[inline(always)]
    pub fn is_arithmetic_sub(&self) -> bool {
        *self == MODE_A::ArithmeticSub
    }
    #[doc = "Checks if the value of the field is `ArithmeticMul`"]
    #[inline(always)]
    pub fn is_arithmetic_mul(&self) -> bool {
        *self == MODE_A::ArithmeticMul
    }
    #[doc = "Checks if the value of the field is `ArithmeticComp`"]
    #[inline(always)]
    pub fn is_arithmetic_comp(&self) -> bool {
        *self == MODE_A::ArithmeticComp
    }
    #[doc = "Checks if the value of the field is `ModularRed`"]
    #[inline(always)]
    pub fn is_modular_red(&self) -> bool {
        *self == MODE_A::ModularRed
    }
    #[doc = "Checks if the value of the field is `ModularAdd`"]
    #[inline(always)]
    pub fn is_modular_add(&self) -> bool {
        *self == MODE_A::ModularAdd
    }
    #[doc = "Checks if the value of the field is `ModularSub`"]
    #[inline(always)]
    pub fn is_modular_sub(&self) -> bool {
        *self == MODE_A::ModularSub
    }
    #[doc = "Checks if the value of the field is `ModularMul`"]
    #[inline(always)]
    pub fn is_modular_mul(&self) -> bool {
        *self == MODE_A::ModularMul
    }
    #[doc = "Checks if the value of the field is `MontgomeryCompScalar`"]
    #[inline(always)]
    pub fn is_montgomery_comp_scalar(&self) -> bool {
        *self == MODE_A::MontgomeryCompScalar
    }
    #[doc = "Checks if the value of the field is `MontgomeryScalar`"]
    #[inline(always)]
    pub fn is_montgomery_scalar(&self) -> bool {
        *self == MODE_A::MontgomeryScalar
    }
    #[doc = "Checks if the value of the field is `Ecdsasign`"]
    #[inline(always)]
    pub fn is_ecdsasign(&self) -> bool {
        *self == MODE_A::Ecdsasign
    }
    #[doc = "Checks if the value of the field is `Ecdsaverif`"]
    #[inline(always)]
    pub fn is_ecdsaverif(&self) -> bool {
        *self == MODE_A::Ecdsaverif
    }
    #[doc = "Checks if the value of the field is `Elliptic`"]
    #[inline(always)]
    pub fn is_elliptic(&self) -> bool {
        *self == MODE_A::Elliptic
    }
}
#[doc = "Field `MODE` writer - PKA operation code"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, MODE_A, 6, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Montgomery parameter computation then modular exponentiation"]
    #[inline(always)]
    pub fn montgomery_comp_exp(self) -> &'a mut W {
        self.variant(MODE_A::MontgomeryCompExp)
    }
    #[doc = "Montgomery parameter computation only"]
    #[inline(always)]
    pub fn montgomery_comp(self) -> &'a mut W {
        self.variant(MODE_A::MontgomeryComp)
    }
    #[doc = "Modular exponentiation only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn montgomery_exp(self) -> &'a mut W {
        self.variant(MODE_A::MontgomeryExp)
    }
    #[doc = "RSA CRT exponentiation"]
    #[inline(always)]
    pub fn rsa(self) -> &'a mut W {
        self.variant(MODE_A::Rsa)
    }
    #[doc = "Modular inversion"]
    #[inline(always)]
    pub fn modular_inv(self) -> &'a mut W {
        self.variant(MODE_A::ModularInv)
    }
    #[doc = "Arithmetic addition"]
    #[inline(always)]
    pub fn arithmetic_add(self) -> &'a mut W {
        self.variant(MODE_A::ArithmeticAdd)
    }
    #[doc = "Arithmetic subtraction"]
    #[inline(always)]
    pub fn arithmetic_sub(self) -> &'a mut W {
        self.variant(MODE_A::ArithmeticSub)
    }
    #[doc = "Arithmetic multiplication"]
    #[inline(always)]
    pub fn arithmetic_mul(self) -> &'a mut W {
        self.variant(MODE_A::ArithmeticMul)
    }
    #[doc = "Arithmetic comparison"]
    #[inline(always)]
    pub fn arithmetic_comp(self) -> &'a mut W {
        self.variant(MODE_A::ArithmeticComp)
    }
    #[doc = "Modular reduction"]
    #[inline(always)]
    pub fn modular_red(self) -> &'a mut W {
        self.variant(MODE_A::ModularRed)
    }
    #[doc = "Modular addition"]
    #[inline(always)]
    pub fn modular_add(self) -> &'a mut W {
        self.variant(MODE_A::ModularAdd)
    }
    #[doc = "Modular subtraction"]
    #[inline(always)]
    pub fn modular_sub(self) -> &'a mut W {
        self.variant(MODE_A::ModularSub)
    }
    #[doc = "Montgomery multiplication"]
    #[inline(always)]
    pub fn modular_mul(self) -> &'a mut W {
        self.variant(MODE_A::ModularMul)
    }
    #[doc = "Montgomery parameter computation then ECC scalar multiplication"]
    #[inline(always)]
    pub fn montgomery_comp_scalar(self) -> &'a mut W {
        self.variant(MODE_A::MontgomeryCompScalar)
    }
    #[doc = "ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn montgomery_scalar(self) -> &'a mut W {
        self.variant(MODE_A::MontgomeryScalar)
    }
    #[doc = "ECDSA sign"]
    #[inline(always)]
    pub fn ecdsasign(self) -> &'a mut W {
        self.variant(MODE_A::Ecdsasign)
    }
    #[doc = "ECDSA verification"]
    #[inline(always)]
    pub fn ecdsaverif(self) -> &'a mut W {
        self.variant(MODE_A::Ecdsaverif)
    }
    #[doc = "Point on elliptic curve Fp check"]
    #[inline(always)]
    pub fn elliptic(self) -> &'a mut W {
        self.variant(MODE_A::Elliptic)
    }
}
#[doc = "Field `PROCENDIE` reader - PROCENDIE"]
pub type PROCENDIE_R = crate::BitReader<PROCENDIE_A>;
#[doc = "PROCENDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDIE_A {
    #[doc = "0: No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    Enabled = 1,
}
impl From<PROCENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: PROCENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PROCENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROCENDIE_A {
        match self.bits {
            false => PROCENDIE_A::Disabled,
            true => PROCENDIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PROCENDIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PROCENDIE_A::Enabled
    }
}
#[doc = "Field `PROCENDIE` writer - PROCENDIE"]
pub type PROCENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PROCENDIE_A, O>;
impl<'a, const O: u8> PROCENDIE_W<'a, O> {
    #[doc = "No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PROCENDIE_A::Disabled)
    }
    #[doc = "An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PROCENDIE_A::Enabled)
    }
}
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub type RAMERRIE_R = crate::BitReader<RAMERRIE_A>;
#[doc = "RAM error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRIE_A {
    #[doc = "0: No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    Enabled = 1,
}
impl From<RAMERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RAMERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMERRIE_A {
        match self.bits {
            false => RAMERRIE_A::Disabled,
            true => RAMERRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMERRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMERRIE_A::Enabled
    }
}
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub type RAMERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RAMERRIE_A, O>;
impl<'a, const O: u8> RAMERRIE_W<'a, O> {
    #[doc = "No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAMERRIE_A::Disabled)
    }
    #[doc = "An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAMERRIE_A::Enabled)
    }
}
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub type ADDRERRIE_R = crate::BitReader<ADDRERRIE_A>;
#[doc = "Address error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRIE_A {
    #[doc = "0: No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    Enabled = 1,
}
impl From<ADDRERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRERRIE_A {
        match self.bits {
            false => ADDRERRIE_A::Disabled,
            true => ADDRERRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRERRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRERRIE_A::Enabled
    }
}
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub type ADDRERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADDRERRIE_A, O>;
impl<'a, const O: u8> ADDRERRIE_W<'a, O> {
    #[doc = "No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRERRIE_A::Disabled)
    }
    #[doc = "An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRERRIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - PKA enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - start the operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - PROCENDIE"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKA enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - start the operation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 17 - PROCENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn procendie(&mut self) -> PROCENDIE_W<17> {
        PROCENDIE_W::new(self)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<19> {
        RAMERRIE_W::new(self)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<20> {
        ADDRERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
