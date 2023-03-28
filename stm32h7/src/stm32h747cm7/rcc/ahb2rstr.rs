#[doc = "Register `AHB2RSTR` reader"]
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RSTR` writer"]
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAMITFRST` reader - CAMITF block reset"]
pub type CAMITFRST_R = crate::BitReader<CAMITFRST_A>;
#[doc = "CAMITF block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAMITFRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<CAMITFRST_A> for bool {
    #[inline(always)]
    fn from(variant: CAMITFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CAMITFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAMITFRST_A> {
        match self.bits {
            true => Some(CAMITFRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CAMITFRST_A::Reset
    }
}
#[doc = "Field `CAMITFRST` writer - CAMITF block reset"]
pub type CAMITFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, CAMITFRST_A, O>;
impl<'a, const O: u8> CAMITFRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CAMITFRST_A::Reset)
    }
}
#[doc = "Field `CRYPTRST` reader - Cryptography block reset"]
pub use CAMITFRST_R as CRYPTRST_R;
#[doc = "Field `HASHRST` reader - Hash block reset"]
pub use CAMITFRST_R as HASHRST_R;
#[doc = "Field `RNGRST` reader - Random Number Generator block reset"]
pub use CAMITFRST_R as RNGRST_R;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 Delay block reset"]
pub use CAMITFRST_R as SDMMC2RST_R;
#[doc = "Field `CRYPTRST` writer - Cryptography block reset"]
pub use CAMITFRST_W as CRYPTRST_W;
#[doc = "Field `HASHRST` writer - Hash block reset"]
pub use CAMITFRST_W as HASHRST_W;
#[doc = "Field `RNGRST` writer - Random Number Generator block reset"]
pub use CAMITFRST_W as RNGRST_W;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 Delay block reset"]
pub use CAMITFRST_W as SDMMC2RST_W;
impl R {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&self) -> CAMITFRST_R {
        CAMITFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    #[must_use]
    pub fn camitfrst(&mut self) -> CAMITFRST_W<0> {
        CAMITFRST_W::new(self)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    #[must_use]
    pub fn cryptrst(&mut self) -> CRYPTRST_W<4> {
        CRYPTRST_W::new(self)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<5> {
        HASHRST_W::new(self)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<6> {
        RNGRST_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<9> {
        SDMMC2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB2 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rstr::R](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
