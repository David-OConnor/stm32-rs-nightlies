#[doc = "Register `PWR_CR3` reader"]
pub struct R(crate::R<PWR_CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR3` writer"]
pub struct W(crate::W<PWR_CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR3_SPEC>;
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
impl From<crate::W<PWR_CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBE` reader - VBE"]
pub type VBE_R = crate::BitReader<bool>;
#[doc = "Field `VBE` writer - VBE"]
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `VBRS` reader - VBRS"]
pub type VBRS_R = crate::BitReader<bool>;
#[doc = "Field `VBRS` writer - VBRS"]
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `DDRSREN` reader - DDRSREN"]
pub type DDRSREN_R = crate::BitReader<bool>;
#[doc = "Field `DDRSREN` writer - DDRSREN"]
pub type DDRSREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `DDRSRDIS` reader - DDRSRDIS"]
pub type DDRSRDIS_R = crate::BitReader<bool>;
#[doc = "Field `DDRSRDIS` writer - DDRSRDIS"]
pub type DDRSRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `DDRRETEN` reader - DDRRETEN"]
pub type DDRRETEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRRETEN` writer - DDRRETEN"]
pub type DDRRETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `POPL` reader - POPL"]
pub type POPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POPL` writer - POPL"]
pub type POPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_CR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `USB33DEN` reader - USB33DEN"]
pub type USB33DEN_R = crate::BitReader<bool>;
#[doc = "Field `USB33DEN` writer - USB33DEN"]
pub type USB33DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `USB33RDY` reader - USB33RDY"]
pub type USB33RDY_R = crate::BitReader<bool>;
#[doc = "Field `REG18EN` reader - REG18EN"]
pub type REG18EN_R = crate::BitReader<bool>;
#[doc = "Field `REG18EN` writer - REG18EN"]
pub type REG18EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `REG18RDY` reader - REG18RDY"]
pub type REG18RDY_R = crate::BitReader<bool>;
#[doc = "Field `REG11EN` reader - REG11EN"]
pub type REG11EN_R = crate::BitReader<bool>;
#[doc = "Field `REG11EN` writer - REG11EN"]
pub type REG11EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR3_SPEC, bool, O>;
#[doc = "Field `REG11RDY` reader - REG11RDY"]
pub type REG11RDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    pub fn ddrsren(&self) -> DDRSREN_R {
        DDRSREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    pub fn ddrsrdis(&self) -> DDRSRDIS_R {
        DDRSRDIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    pub fn ddrreten(&self) -> DDRRETEN_R {
        DDRRETEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    pub fn popl(&self) -> POPL_R {
        POPL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - USB33RDY"]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    pub fn reg18en(&self) -> REG18EN_R {
        REG18EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - REG18RDY"]
    #[inline(always)]
    pub fn reg18rdy(&self) -> REG18RDY_R {
        REG18RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    pub fn reg11en(&self) -> REG11EN_R {
        REG11EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - REG11RDY"]
    #[inline(always)]
    pub fn reg11rdy(&self) -> REG11RDY_R {
        REG11RDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrsren(&mut self) -> DDRSREN_W<10> {
        DDRSREN_W::new(self)
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    #[must_use]
    pub fn ddrsrdis(&mut self) -> DDRSRDIS_W<11> {
        DDRSRDIS_W::new(self)
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrreten(&mut self) -> DDRRETEN_W<12> {
        DDRRETEN_W::new(self)
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    #[must_use]
    pub fn popl(&mut self) -> POPL_W<17> {
        POPL_W::new(self)
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<24> {
        USB33DEN_W::new(self)
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    #[must_use]
    pub fn reg18en(&mut self) -> REG18EN_W<28> {
        REG18EN_W::new(self)
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    #[must_use]
    pub fn reg11en(&mut self) -> REG11EN_W<30> {
        REG11EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr3](index.html) module"]
pub struct PWR_CR3_SPEC;
impl crate::RegisterSpec for PWR_CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr3::R](R) reader structure"]
impl crate::Readable for PWR_CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr3::W](W) writer structure"]
impl crate::Writable for PWR_CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0x5000_0000"]
impl crate::Resettable for PWR_CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000_0000;
}
