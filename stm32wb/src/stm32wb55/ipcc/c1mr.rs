#[doc = "Register `C1MR` reader"]
pub struct R(crate::R<C1MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1MR` writer"]
pub struct W(crate::W<C1MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1MR_SPEC>;
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
impl From<crate::W<C1MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1OM` reader - processor 1 Receive channel 1 occupied interrupt enable"]
pub type CH1OM_R = crate::BitReader<bool>;
#[doc = "Field `CH1OM` writer - processor 1 Receive channel 1 occupied interrupt enable"]
pub type CH1OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH2OM` reader - processor 1 Receive channel 2 occupied interrupt enable"]
pub type CH2OM_R = crate::BitReader<bool>;
#[doc = "Field `CH2OM` writer - processor 1 Receive channel 2 occupied interrupt enable"]
pub type CH2OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH3OM` reader - processor 1 Receive channel 3 occupied interrupt enable"]
pub type CH3OM_R = crate::BitReader<bool>;
#[doc = "Field `CH3OM` writer - processor 1 Receive channel 3 occupied interrupt enable"]
pub type CH3OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH4OM` reader - processor 1 Receive channel 4 occupied interrupt enable"]
pub type CH4OM_R = crate::BitReader<bool>;
#[doc = "Field `CH4OM` writer - processor 1 Receive channel 4 occupied interrupt enable"]
pub type CH4OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH5OM` reader - processor 1 Receive channel 5 occupied interrupt enable"]
pub type CH5OM_R = crate::BitReader<bool>;
#[doc = "Field `CH5OM` writer - processor 1 Receive channel 5 occupied interrupt enable"]
pub type CH5OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH6OM` reader - processor 1 Receive channel 6 occupied interrupt enable"]
pub type CH6OM_R = crate::BitReader<bool>;
#[doc = "Field `CH6OM` writer - processor 1 Receive channel 6 occupied interrupt enable"]
pub type CH6OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH1FM` reader - processor 1 Transmit channel 1 free interrupt mask"]
pub type CH1FM_R = crate::BitReader<bool>;
#[doc = "Field `CH1FM` writer - processor 1 Transmit channel 1 free interrupt mask"]
pub type CH1FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH2FM` reader - processor 1 Transmit channel 2 free interrupt mask"]
pub type CH2FM_R = crate::BitReader<bool>;
#[doc = "Field `CH2FM` writer - processor 1 Transmit channel 2 free interrupt mask"]
pub type CH2FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH3FM` reader - processor 1 Transmit channel 3 free interrupt mask"]
pub type CH3FM_R = crate::BitReader<bool>;
#[doc = "Field `CH3FM` writer - processor 1 Transmit channel 3 free interrupt mask"]
pub type CH3FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH4FM` reader - processor 1 Transmit channel 4 free interrupt mask"]
pub type CH4FM_R = crate::BitReader<bool>;
#[doc = "Field `CH4FM` writer - processor 1 Transmit channel 4 free interrupt mask"]
pub type CH4FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH5FM` reader - processor 1 Transmit channel 5 free interrupt mask"]
pub type CH5FM_R = crate::BitReader<bool>;
#[doc = "Field `CH5FM` writer - processor 1 Transmit channel 5 free interrupt mask"]
pub type CH5FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
#[doc = "Field `CH6FM` reader - processor 1 Transmit channel 6 free interrupt mask"]
pub type CH6FM_R = crate::BitReader<bool>;
#[doc = "Field `CH6FM` writer - processor 1 Transmit channel 6 free interrupt mask"]
pub type CH6FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - processor 1 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - processor 1 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - processor 1 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - processor 1 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - processor 1 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - processor 1 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - processor 1 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - processor 1 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - processor 1 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - processor 1 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - processor 1 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - processor 1 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om(&mut self) -> CH1OM_W<0> {
        CH1OM_W::new(self)
    }
    #[doc = "Bit 1 - processor 1 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om(&mut self) -> CH2OM_W<1> {
        CH2OM_W::new(self)
    }
    #[doc = "Bit 2 - processor 1 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<2> {
        CH3OM_W::new(self)
    }
    #[doc = "Bit 3 - processor 1 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om(&mut self) -> CH4OM_W<3> {
        CH4OM_W::new(self)
    }
    #[doc = "Bit 4 - processor 1 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om(&mut self) -> CH5OM_W<4> {
        CH5OM_W::new(self)
    }
    #[doc = "Bit 5 - processor 1 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om(&mut self) -> CH6OM_W<5> {
        CH6OM_W::new(self)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fm(&mut self) -> CH1FM_W<16> {
        CH1FM_W::new(self)
    }
    #[doc = "Bit 17 - processor 1 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch2fm(&mut self) -> CH2FM_W<17> {
        CH2FM_W::new(self)
    }
    #[doc = "Bit 18 - processor 1 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch3fm(&mut self) -> CH3FM_W<18> {
        CH3FM_W::new(self)
    }
    #[doc = "Bit 19 - processor 1 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch4fm(&mut self) -> CH4FM_W<19> {
        CH4FM_W::new(self)
    }
    #[doc = "Bit 20 - processor 1 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch5fm(&mut self) -> CH5FM_W<20> {
        CH5FM_W::new(self)
    }
    #[doc = "Bit 21 - processor 1 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch6fm(&mut self) -> CH6FM_W<21> {
        CH6FM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask register CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1mr](index.html) module"]
pub struct C1MR_SPEC;
impl crate::RegisterSpec for C1MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1mr::R](R) reader structure"]
impl crate::Readable for C1MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1mr::W](W) writer structure"]
impl crate::Writable for C1MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1MR to value 0xffff_ffff"]
impl crate::Resettable for C1MR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
