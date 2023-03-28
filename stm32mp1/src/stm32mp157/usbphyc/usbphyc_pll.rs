#[doc = "Register `USBPHYC_PLL` reader"]
pub struct R(crate::R<USBPHYC_PLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_PLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_PLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_PLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYC_PLL` writer"]
pub struct W(crate::W<USBPHYC_PLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_PLL_SPEC>;
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
impl From<crate::W<USBPHYC_PLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_PLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLNDIV` reader - PLLNDIV"]
pub type PLLNDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLNDIV` writer - PLLNDIV"]
pub type PLLNDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_PLL_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLLODF` reader - PLLODF"]
pub type PLLODF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLODF` writer - PLLODF"]
pub type PLLODF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_PLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PLLFRACIN` reader - PLLFRACIN"]
pub type PLLFRACIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLLFRACIN` writer - PLLFRACIN"]
pub type PLLFRACIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_PLL_SPEC, u16, u16, 16, O>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_PLL_SPEC, bool, O>;
#[doc = "Field `PLLSTRB` reader - PLLSTRB"]
pub type PLLSTRB_R = crate::BitReader<bool>;
#[doc = "Field `PLLSTRB` writer - PLLSTRB"]
pub type PLLSTRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_PLL_SPEC, bool, O>;
#[doc = "Field `PLLSTRBYP` reader - PLLSTRBYP"]
pub type PLLSTRBYP_R = crate::BitReader<bool>;
#[doc = "Field `PLLSTRBYP` writer - PLLSTRBYP"]
pub type PLLSTRBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_PLL_SPEC, bool, O>;
#[doc = "Field `PLLFRACCTL` reader - PLLFRACCTL"]
pub type PLLFRACCTL_R = crate::BitReader<bool>;
#[doc = "Field `PLLFRACCTL` writer - PLLFRACCTL"]
pub type PLLFRACCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_PLL_SPEC, bool, O>;
#[doc = "Field `PLLDITHEN0` reader - PLLDITHEN0"]
pub type PLLDITHEN0_R = crate::BitReader<bool>;
#[doc = "Field `PLLDITHEN0` writer - PLLDITHEN0"]
pub type PLLDITHEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_PLL_SPEC, bool, O>;
#[doc = "Field `PLLDITHEN1` reader - PLLDITHEN1"]
pub type PLLDITHEN1_R = crate::BitReader<bool>;
#[doc = "Field `PLLDITHEN1` writer - PLLDITHEN1"]
pub type PLLDITHEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_PLL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    pub fn pllndiv(&self) -> PLLNDIV_R {
        PLLNDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    pub fn pllodf(&self) -> PLLODF_R {
        PLLODF_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    pub fn pllfracin(&self) -> PLLFRACIN_R {
        PLLFRACIN_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    pub fn pllstrb(&self) -> PLLSTRB_R {
        PLLSTRB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    pub fn pllstrbyp(&self) -> PLLSTRBYP_R {
        PLLSTRBYP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    pub fn pllfracctl(&self) -> PLLFRACCTL_R {
        PLLFRACCTL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    pub fn plldithen0(&self) -> PLLDITHEN0_R {
        PLLDITHEN0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    pub fn plldithen1(&self) -> PLLDITHEN1_R {
        PLLDITHEN1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    #[must_use]
    pub fn pllndiv(&mut self) -> PLLNDIV_W<0> {
        PLLNDIV_W::new(self)
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    #[must_use]
    pub fn pllodf(&mut self) -> PLLODF_W<7> {
        PLLODF_W::new(self)
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    #[must_use]
    pub fn pllfracin(&mut self) -> PLLFRACIN_W<10> {
        PLLFRACIN_W::new(self)
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<26> {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    #[must_use]
    pub fn pllstrb(&mut self) -> PLLSTRB_W<27> {
        PLLSTRB_W::new(self)
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    #[must_use]
    pub fn pllstrbyp(&mut self) -> PLLSTRBYP_W<28> {
        PLLSTRBYP_W::new(self)
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    #[must_use]
    pub fn pllfracctl(&mut self) -> PLLFRACCTL_W<29> {
        PLLFRACCTL_W::new(self)
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    #[must_use]
    pub fn plldithen0(&mut self) -> PLLDITHEN0_W<30> {
        PLLDITHEN0_W::new(self)
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    #[must_use]
    pub fn plldithen1(&mut self) -> PLLDITHEN1_W<31> {
        PLLDITHEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the PLL of the HS PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_pll](index.html) module"]
pub struct USBPHYC_PLL_SPEC;
impl crate::RegisterSpec for USBPHYC_PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphyc_pll::R](R) reader structure"]
impl crate::Readable for USBPHYC_PLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyc_pll::W](W) writer structure"]
impl crate::Writable for USBPHYC_PLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHYC_PLL to value 0xc000_0000"]
impl crate::Resettable for USBPHYC_PLL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
