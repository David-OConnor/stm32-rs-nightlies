#[doc = "Register `ETH_MACLCSR` reader"]
pub struct R(crate::R<ETH_MACLCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLCSR` writer"]
pub struct W(crate::W<ETH_MACLCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLCSR_SPEC>;
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
impl From<crate::W<ETH_MACLCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLPIEN` reader - TLPIEN"]
pub type TLPIEN_R = crate::BitReader<bool>;
#[doc = "Field `TLPIEX` reader - TLPIEX"]
pub type TLPIEX_R = crate::BitReader<bool>;
#[doc = "Field `RLPIEN` reader - RLPIEN"]
pub type RLPIEN_R = crate::BitReader<bool>;
#[doc = "Field `RLPIEX` reader - RLPIEX"]
pub type RLPIEX_R = crate::BitReader<bool>;
#[doc = "Field `TLPIST` reader - TLPIST"]
pub type TLPIST_R = crate::BitReader<bool>;
#[doc = "Field `RLPIST` reader - RLPIST"]
pub type RLPIST_R = crate::BitReader<bool>;
#[doc = "Field `LPIEN` reader - LPIEN"]
pub type LPIEN_R = crate::BitReader<bool>;
#[doc = "Field `LPIEN` writer - LPIEN"]
pub type LPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACLCSR_SPEC, bool, O>;
#[doc = "Field `PLS` reader - PLS"]
pub type PLS_R = crate::BitReader<bool>;
#[doc = "Field `PLS` writer - PLS"]
pub type PLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACLCSR_SPEC, bool, O>;
#[doc = "Field `PLSEN` reader - PLSEN"]
pub type PLSEN_R = crate::BitReader<bool>;
#[doc = "Field `PLSEN` writer - PLSEN"]
pub type PLSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACLCSR_SPEC, bool, O>;
#[doc = "Field `LPITXA` reader - LPITXA"]
pub type LPITXA_R = crate::BitReader<bool>;
#[doc = "Field `LPITXA` writer - LPITXA"]
pub type LPITXA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACLCSR_SPEC, bool, O>;
#[doc = "Field `LPITE` reader - LPITE"]
pub type LPITE_R = crate::BitReader<bool>;
#[doc = "Field `LPITE` writer - LPITE"]
pub type LPITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACLCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TLPIEN"]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TLPIEX"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RLPIEN"]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RLPIEX"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TLPIST"]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RLPIST"]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LPIEN_W<16> {
        LPIEN_W::new(self)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<17> {
        PLS_W::new(self)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    #[must_use]
    pub fn plsen(&mut self) -> PLSEN_W<18> {
        PLSEN_W::new(self)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LPITXA_W<19> {
        LPITXA_W::new(self)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    #[must_use]
    pub fn lpite(&mut self) -> LPITE_W<20> {
        LPITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maclcsr](index.html) module"]
pub struct ETH_MACLCSR_SPEC;
impl crate::RegisterSpec for ETH_MACLCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maclcsr::R](R) reader structure"]
impl crate::Readable for ETH_MACLCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maclcsr::W](W) writer structure"]
impl crate::Writable for ETH_MACLCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACLCSR to value 0"]
impl crate::Resettable for ETH_MACLCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
