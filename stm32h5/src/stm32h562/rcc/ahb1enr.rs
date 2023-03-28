#[doc = "Register `AHB1ENR` reader"]
pub struct R(crate::R<AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1ENR` writer"]
pub struct W(crate::W<AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1ENR_SPEC>;
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
impl From<crate::W<AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPDMA1EN` reader - GPDMA1 clock enable Set and reset by software."]
pub type GPDMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `GPDMA1EN` writer - GPDMA1 clock enable Set and reset by software."]
pub type GPDMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `GPDMA2EN` reader - GPDMA2 clock enable Set and reset by software."]
pub type GPDMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `GPDMA2EN` writer - GPDMA2 clock enable Set and reset by software."]
pub type GPDMA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `FLITFEN` reader - Flash interface clock enable Set and reset by software."]
pub type FLITFEN_R = crate::BitReader<bool>;
#[doc = "Field `FLITFEN` writer - Flash interface clock enable Set and reset by software."]
pub type FLITFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - CRC clock enable Set and reset by software."]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRC clock enable Set and reset by software."]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `CORDICEN` reader - CORDIC clock enable Set and reset by software."]
pub type CORDICEN_R = crate::BitReader<bool>;
#[doc = "Field `CORDICEN` writer - CORDIC clock enable Set and reset by software."]
pub type CORDICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `FMACEN` reader - FMAC clock enable Set and reset by software."]
pub type FMACEN_R = crate::BitReader<bool>;
#[doc = "Field `FMACEN` writer - FMAC clock enable Set and reset by software."]
pub type FMACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `RAMCFGEN` reader - RAMCFG clock enable Set and reset by software."]
pub type RAMCFGEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGEN` writer - RAMCFG clock enable Set and reset by software."]
pub type RAMCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `TZSC1EN` reader - TZSC1 clock enable Set and reset by software"]
pub type TZSC1EN_R = crate::BitReader<bool>;
#[doc = "Field `TZSC1EN` writer - TZSC1 clock enable Set and reset by software"]
pub type TZSC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `BKPRAMEN` reader - BKPRAM clock enable Set and reset by software"]
pub type BKPRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `BKPRAMEN` writer - BKPRAM clock enable Set and reset by software"]
pub type BKPRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `DCACHEEN` reader - DCACHE clock enable Set and reset by software"]
pub type DCACHEEN_R = crate::BitReader<bool>;
#[doc = "Field `DCACHEEN` writer - DCACHE clock enable Set and reset by software"]
pub type DCACHEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `SRAM1EN` reader - SRAM1 clock enable Set and reset by software."]
pub type SRAM1EN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1EN` writer - SRAM1 clock enable Set and reset by software."]
pub type SRAM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpdma2en(&self) -> GPDMA2EN_R {
        GPDMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FMAC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - TZSC1 clock enable Set and reset by software"]
    #[inline(always)]
    pub fn tzsc1en(&self) -> TZSC1EN_R {
        TZSC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPRAM clock enable Set and reset by software"]
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DCACHE clock enable Set and reset by software"]
    #[inline(always)]
    pub fn dcacheen(&self) -> DCACHEEN_R {
        DCACHEEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<0> {
        GPDMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2en(&mut self) -> GPDMA2EN_W<1> {
        GPDMA2EN_W::new(self)
    }
    #[doc = "Bit 8 - Flash interface clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<8> {
        FLITFEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CORDICEN_W<14> {
        CORDICEN_W::new(self)
    }
    #[doc = "Bit 15 - FMAC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FMACEN_W<15> {
        FMACEN_W::new(self)
    }
    #[doc = "Bit 17 - RAMCFG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<17> {
        RAMCFGEN_W::new(self)
    }
    #[doc = "Bit 24 - TZSC1 clock enable Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn tzsc1en(&mut self) -> TZSC1EN_W<24> {
        TZSC1EN_W::new(self)
    }
    #[doc = "Bit 28 - BKPRAM clock enable Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<28> {
        BKPRAMEN_W::new(self)
    }
    #[doc = "Bit 30 - DCACHE clock enable Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn dcacheen(&mut self) -> DCACHEEN_W<30> {
        DCACHEEN_W::new(self)
    }
    #[doc = "Bit 31 - SRAM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<31> {
        SRAM1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB1 peripherals clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](index.html) module"]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1enr::R](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0xd000_0100"]
impl crate::Resettable for AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xd000_0100;
}
