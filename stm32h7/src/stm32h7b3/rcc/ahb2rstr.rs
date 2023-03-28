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
#[doc = "Field `DCMI_PSSIRST` reader - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSIRST_R = crate::BitReader<DCMI_PSSIRST_A>;
#[doc = "digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSIRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DCMI_PSSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMI_PSSIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCMI_PSSIRST_A> {
        match self.bits {
            true => Some(DCMI_PSSIRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMI_PSSIRST_A::Reset
    }
}
#[doc = "Field `DCMI_PSSIRST` writer - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSIRST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB2RSTR_SPEC, DCMI_PSSIRST_A, O>;
impl<'a, const O: u8> DCMI_PSSIRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCMI_PSSIRST_A::Reset)
    }
}
#[doc = "Field `HSEMRST` reader - HSEM block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as HSEMRST_R;
#[doc = "Field `CRYPTRST` reader - cryptography block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as CRYPTRST_R;
#[doc = "Field `HASHRST` reader - hash block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as HASHRST_R;
#[doc = "Field `RNGRST` reader - random number generator block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as RNGRST_R;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub use DCMI_PSSIRST_R as SDMMC2RST_R;
#[doc = "Field `BDMA1RST` reader - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub use DCMI_PSSIRST_R as BDMA1RST_R;
#[doc = "Field `HSEMRST` writer - HSEM block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as HSEMRST_W;
#[doc = "Field `CRYPTRST` writer - cryptography block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as CRYPTRST_W;
#[doc = "Field `HASHRST` writer - hash block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as HASHRST_W;
#[doc = "Field `RNGRST` writer - random number generator block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as RNGRST_W;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub use DCMI_PSSIRST_W as SDMMC2RST_W;
#[doc = "Field `BDMA1RST` writer - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub use DCMI_PSSIRST_W as BDMA1RST_W;
impl R {
    #[doc = "Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - HSEM block reset Set and reset by software."]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - cryptography block reset Set and reset by software."]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hash block reset Set and reset by software."]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - random number generator block reset Set and reset by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    pub fn bdma1rst(&self) -> BDMA1RST_R {
        BDMA1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<0> {
        DCMI_PSSIRST_W::new(self)
    }
    #[doc = "Bit 2 - HSEM block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<2> {
        HSEMRST_W::new(self)
    }
    #[doc = "Bit 4 - cryptography block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cryptrst(&mut self) -> CRYPTRST_W<4> {
        CRYPTRST_W::new(self)
    }
    #[doc = "Bit 5 - hash block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<5> {
        HASHRST_W::new(self)
    }
    #[doc = "Bit 6 - random number generator block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<6> {
        RNGRST_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<9> {
        SDMMC2RST_W::new(self)
    }
    #[doc = "Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn bdma1rst(&mut self) -> BDMA1RST_W<11> {
        BDMA1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
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
