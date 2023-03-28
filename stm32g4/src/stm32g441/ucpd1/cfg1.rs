#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBITCLKDIV` reader - HBITCLKDIV"]
pub type HBITCLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBITCLKDIV` writer - HBITCLKDIV"]
pub type HBITCLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 6, O>;
#[doc = "Field `IFRGAP` reader - IFRGAP"]
pub type IFRGAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFRGAP` writer - IFRGAP"]
pub type IFRGAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRANSWIN` reader - TRANSWIN"]
pub type TRANSWIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANSWIN` writer - TRANSWIN"]
pub type TRANSWIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `PSC_USBPDCLK` reader - PSC_USBPDCLK"]
pub type PSC_USBPDCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC_USBPDCLK` writer - PSC_USBPDCLK"]
pub type PSC_USBPDCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXORDSETEN` reader - RXORDSETEN"]
pub type RXORDSETEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXORDSETEN` writer - RXORDSETEN"]
pub type RXORDSETEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG1_SPEC, u16, u16, 9, O>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
#[doc = "Field `UCPDEN` reader - UCPDEN"]
pub type UCPDEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPDEN` writer - UCPDEN"]
pub type UCPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W<0> {
        HBITCLKDIV_W::new(self)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    #[must_use]
    pub fn ifrgap(&mut self) -> IFRGAP_W<6> {
        IFRGAP_W::new(self)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    #[must_use]
    pub fn transwin(&mut self) -> TRANSWIN_W<11> {
        TRANSWIN_W::new(self)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    #[must_use]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W<17> {
        PSC_USBPDCLK_W::new(self)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W<20> {
        RXORDSETEN_W::new(self)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<29> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 30 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<30> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    #[must_use]
    pub fn ucpden(&mut self) -> UCPDEN_W<31> {
        UCPDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
