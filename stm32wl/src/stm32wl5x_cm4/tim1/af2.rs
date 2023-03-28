#[doc = "Register `AF2` reader"]
pub struct R(crate::R<AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF2` writer"]
pub struct W(crate::W<AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF2_SPEC>;
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
impl From<crate::W<AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub type BK2INE_R = crate::BitReader<BK2INE_A>;
#[doc = "BRK2 BKIN input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE_A {
    #[doc = "0: BKIN input disabled"]
    Disabled = 0,
    #[doc = "1: BKIN input enabled"]
    Enabled = 1,
}
impl From<BK2INE_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INE_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2INE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INE_A {
        match self.bits {
            false => BK2INE_A::Disabled,
            true => BK2INE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2INE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2INE_A::Enabled
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub type BK2INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, BK2INE_A, O>;
impl<'a, const O: u8> BK2INE_W<'a, O> {
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2INE_A::Disabled)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2INE_A::Enabled)
    }
}
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type BK2CMP1E_R = crate::BitReader<BK2CMP1E_A>;
#[doc = "BRK2 COMP1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1E_A {
    #[doc = "0: COMP1 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP1 input enabled"]
    Enabled = 1,
}
impl From<BK2CMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP1E_A {
        match self.bits {
            false => BK2CMP1E_A::Disabled,
            true => BK2CMP1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP1E_A::Enabled
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type BK2CMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, BK2CMP1E_A, O>;
impl<'a, const O: u8> BK2CMP1E_W<'a, O> {
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2CMP1E_A::Disabled)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2CMP1E_A::Enabled)
    }
}
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type BK2CMP2E_R = crate::BitReader<BK2CMP2E_A>;
#[doc = "BRK2 COMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2E_A {
    #[doc = "0: COMP2 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP2 input enabled"]
    Enabled = 1,
}
impl From<BK2CMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP2E_A {
        match self.bits {
            false => BK2CMP2E_A::Disabled,
            true => BK2CMP2E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP2E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP2E_A::Enabled
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type BK2CMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, BK2CMP2E_A, O>;
impl<'a, const O: u8> BK2CMP2E_W<'a, O> {
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2CMP2E_A::Disabled)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2CMP2E_A::Enabled)
    }
}
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity"]
pub type BK2INP_R = crate::BitReader<BK2INP_A>;
#[doc = "BRK2 BKIN2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP_A {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BK2INP_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INP_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2INP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INP_A {
        match self.bits {
            false => BK2INP_A::NotInverted,
            true => BK2INP_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2INP_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2INP_A::Inverted
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity"]
pub type BK2INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, BK2INP_A, O>;
impl<'a, const O: u8> BK2INP_W<'a, O> {
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BK2INP_A::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BK2INP_A::Inverted)
    }
}
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_R = crate::BitReader<BK2CMP1P_A>;
#[doc = "BRK2 COMP1 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1P_A {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BK2CMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP1P_A {
        match self.bits {
            false => BK2CMP1P_A::NotInverted,
            true => BK2CMP1P_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP1P_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP1P_A::Inverted
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, BK2CMP1P_A, O>;
impl<'a, const O: u8> BK2CMP1P_W<'a, O> {
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BK2CMP1P_A::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BK2CMP1P_A::Inverted)
    }
}
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_R = crate::BitReader<BK2CMP2P_A>;
#[doc = "BRK2 COMP2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2P_A {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BK2CMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP2P_A {
        match self.bits {
            false => BK2CMP2P_A::NotInverted,
            true => BK2CMP2P_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP2P_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP2P_A::Inverted
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, BK2CMP2P_A, O>;
impl<'a, const O: u8> BK2CMP2P_W<'a, O> {
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BK2CMP2P_A::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BK2CMP2P_A::Inverted)
    }
}
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<0> {
        BK2INE_W::new(self)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<1> {
        BK2CMP1E_W::new(self)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<2> {
        BK2CMP2E_W::new(self)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<9> {
        BK2INP_W::new(self)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<10> {
        BK2CMP1P_W::new(self)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<11> {
        BK2CMP2P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](index.html) module"]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af2::R](R) reader structure"]
impl crate::Readable for AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af2::W](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AF2 to value 0x01"]
impl crate::Resettable for AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
