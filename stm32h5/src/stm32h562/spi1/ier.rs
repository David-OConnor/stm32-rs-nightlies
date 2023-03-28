#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPIE` reader - RXP interrupt enable"]
pub type RXPIE_R = crate::BitReader<bool>;
#[doc = "Field `RXPIE` writer - RXP interrupt enable"]
pub type RXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TXPIE` reader - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
pub type TXPIE_R = crate::BitReader<bool>;
#[doc = "Field `TXPIE` writer - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
pub type TXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `DXPIE` reader - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
pub type DXPIE_R = crate::BitReader<bool>;
#[doc = "Field `DXPIE` writer - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
pub type DXPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_R = crate::BitReader<bool>;
#[doc = "Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable"]
pub type EOTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TXTFIE` reader - TXTFIE interrupt enable"]
pub type TXTFIE_R = crate::BitReader<bool>;
#[doc = "Field `TXTFIE` writer - TXTFIE interrupt enable"]
pub type TXTFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `UDRIE` reader - UDR interrupt enable"]
pub type UDRIE_R = crate::BitReader<bool>;
#[doc = "Field `UDRIE` writer - UDR interrupt enable"]
pub type UDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `OVRIE` reader - OVR interrupt enable"]
pub type OVRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRIE` writer - OVR interrupt enable"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `CRCEIE` reader - CRC error interrupt enable"]
pub type CRCEIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCEIE` writer - CRC error interrupt enable"]
pub type CRCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TIFREIE` reader - TIFRE interrupt enable"]
pub type TIFREIE_R = crate::BitReader<bool>;
#[doc = "Field `TIFREIE` writer - TIFRE interrupt enable"]
pub type TIFREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `MODFIE` reader - mode Fault interrupt enable"]
pub type MODFIE_R = crate::BitReader<bool>;
#[doc = "Field `MODFIE` writer - mode Fault interrupt enable"]
pub type MODFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXP interrupt enable"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC error interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXP interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxpie(&mut self) -> RXPIE_W<0> {
        RXPIE_W::new(self)
    }
    #[doc = "Bit 1 - TXP interrupt enable TXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    #[must_use]
    pub fn txpie(&mut self) -> TXPIE_W<1> {
        TXPIE_W::new(self)
    }
    #[doc = "Bit 2 - DXP interrupt enabled DXPIE is set by software and cleared by TXTF flag set event."]
    #[inline(always)]
    #[must_use]
    pub fn dxpie(&mut self) -> DXPIE_W<2> {
        DXPIE_W::new(self)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eotie(&mut self) -> EOTIE_W<3> {
        EOTIE_W::new(self)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txtfie(&mut self) -> TXTFIE_W<4> {
        TXTFIE_W::new(self)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn udrie(&mut self) -> UDRIE_W<5> {
        UDRIE_W::new(self)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<6> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 7 - CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<7> {
        CRCEIE_W::new(self)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tifreie(&mut self) -> TIFREIE_W<8> {
        TIFREIE_W::new(self)
    }
    #[doc = "Bit 9 - mode Fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn modfie(&mut self) -> MODFIE_W<9> {
        MODFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
