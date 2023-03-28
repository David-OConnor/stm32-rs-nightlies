#[doc = "Register `ETH_MACPPSCR` reader"]
pub struct R(crate::R<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSCR` writer"]
pub struct W(crate::W<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSCR_SPEC>;
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
impl From<crate::W<ETH_MACPPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSCTRL` reader - PPSCTRL"]
pub type PPSCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCTRL` writer - PPSCTRL"]
pub type PPSCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACPPSCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPSEN0` reader - PPSEN0"]
pub type PPSEN0_R = crate::BitReader<bool>;
#[doc = "Field `PPSEN0` writer - PPSEN0"]
pub type PPSEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPPSCR_SPEC, bool, O>;
#[doc = "Field `TRGTMODSEL0` reader - TRGTMODSEL0"]
pub type TRGTMODSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL0` writer - TRGTMODSEL0"]
pub type TRGTMODSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<0> {
        PPSCTRL_W::new(self)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<4> {
        PPSEN0_W::new(self)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<5> {
        TRGTMODSEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppscr](index.html) module"]
pub struct ETH_MACPPSCR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppscr::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppscr::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSCR to value 0"]
impl crate::Resettable for ETH_MACPPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
