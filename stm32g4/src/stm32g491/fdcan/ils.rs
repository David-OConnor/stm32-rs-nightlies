#[doc = "Register `ILS` reader"]
pub struct R(crate::R<ILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILS` writer"]
pub struct W(crate::W<ILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILS_SPEC>;
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
impl From<crate::W<ILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO0` reader - RX FIFO bit grouping the following interruption"]
pub type RXFIFO0_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO0` writer - RX FIFO bit grouping the following interruption"]
pub type RXFIFO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
#[doc = "Field `RXFIFO1` reader - RX FIFO bit grouping the following interruption"]
pub type RXFIFO1_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO1` writer - RX FIFO bit grouping the following interruption"]
pub type RXFIFO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
#[doc = "Field `SMSG` reader - Status message bit grouping the following interruption"]
pub type SMSG_R = crate::BitReader<bool>;
#[doc = "Field `SMSG` writer - Status message bit grouping the following interruption"]
pub type SMSG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
#[doc = "Field `TFERR` reader - TX FIFO error grouping the following interruption"]
pub type TFERR_R = crate::BitReader<bool>;
#[doc = "Field `TFERR` writer - TX FIFO error grouping the following interruption"]
pub type TFERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
#[doc = "Field `MISC` reader - Interrupt regrouping the following interruption"]
pub type MISC_R = crate::BitReader<bool>;
#[doc = "Field `MISC` writer - Interrupt regrouping the following interruption"]
pub type MISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
#[doc = "Field `BERR` reader - Bit and line error grouping the following interruption"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - Bit and line error grouping the following interruption"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
#[doc = "Field `PERR` reader - Protocol error grouping the following interruption"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - Protocol error grouping the following interruption"]
pub type PERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    pub fn rxfifo0(&self) -> RXFIFO0_R {
        RXFIFO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    pub fn rxfifo1(&self) -> RXFIFO1_R {
        RXFIFO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption"]
    #[inline(always)]
    pub fn smsg(&self) -> SMSG_R {
        SMSG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO error grouping the following interruption"]
    #[inline(always)]
    pub fn tferr(&self) -> TFERR_R {
        TFERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption"]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo0(&mut self) -> RXFIFO0_W<0> {
        RXFIFO0_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo1(&mut self) -> RXFIFO1_W<1> {
        RXFIFO1_W::new(self)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn smsg(&mut self) -> SMSG_W<2> {
        SMSG_W::new(self)
    }
    #[doc = "Bit 3 - TX FIFO error grouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn tferr(&mut self) -> TFERR_W<3> {
        TFERR_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn misc(&mut self) -> MISC_W<4> {
        MISC_W::new(self)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<5> {
        BERR_W::new(self)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<6> {
        PERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ils](index.html) module"]
pub struct ILS_SPEC;
impl crate::RegisterSpec for ILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ils::R](R) reader structure"]
impl crate::Readable for ILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ils::W](W) writer structure"]
impl crate::Writable for ILS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILS to value 0"]
impl crate::Resettable for ILS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
