#[doc = "Register `ISR0` reader"]
pub struct R(crate::R<ISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR0` writer"]
pub struct W(crate::W<ISR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR0_SPEC>;
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
impl From<crate::W<ISR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AE0` reader - Acknowledge error 0"]
pub type AE0_R = crate::BitReader<bool>;
#[doc = "Field `AE0` writer - Acknowledge error 0"]
pub type AE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE1` reader - Acknowledge error 1"]
pub type AE1_R = crate::BitReader<bool>;
#[doc = "Field `AE1` writer - Acknowledge error 1"]
pub type AE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE2` reader - Acknowledge error 2"]
pub type AE2_R = crate::BitReader<bool>;
#[doc = "Field `AE2` writer - Acknowledge error 2"]
pub type AE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE3` reader - Acknowledge error 3"]
pub type AE3_R = crate::BitReader<bool>;
#[doc = "Field `AE3` writer - Acknowledge error 3"]
pub type AE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE4` reader - Acknowledge error 4"]
pub type AE4_R = crate::BitReader<bool>;
#[doc = "Field `AE4` writer - Acknowledge error 4"]
pub type AE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE5` reader - Acknowledge error 5"]
pub type AE5_R = crate::BitReader<bool>;
#[doc = "Field `AE5` writer - Acknowledge error 5"]
pub type AE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE6` reader - Acknowledge error 6"]
pub type AE6_R = crate::BitReader<bool>;
#[doc = "Field `AE6` writer - Acknowledge error 6"]
pub type AE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE7` reader - Acknowledge error 7"]
pub type AE7_R = crate::BitReader<bool>;
#[doc = "Field `AE7` writer - Acknowledge error 7"]
pub type AE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE8` reader - Acknowledge error 8"]
pub type AE8_R = crate::BitReader<bool>;
#[doc = "Field `AE8` writer - Acknowledge error 8"]
pub type AE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE9` reader - Acknowledge error 9"]
pub type AE9_R = crate::BitReader<bool>;
#[doc = "Field `AE9` writer - Acknowledge error 9"]
pub type AE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE10` reader - Acknowledge error 10"]
pub type AE10_R = crate::BitReader<bool>;
#[doc = "Field `AE10` writer - Acknowledge error 10"]
pub type AE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE11` reader - Acknowledge error 11"]
pub type AE11_R = crate::BitReader<bool>;
#[doc = "Field `AE11` writer - Acknowledge error 11"]
pub type AE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE12` reader - Acknowledge error 12"]
pub type AE12_R = crate::BitReader<bool>;
#[doc = "Field `AE12` writer - Acknowledge error 12"]
pub type AE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE13` reader - Acknowledge error 13"]
pub type AE13_R = crate::BitReader<bool>;
#[doc = "Field `AE13` writer - Acknowledge error 13"]
pub type AE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE14` reader - Acknowledge error 14"]
pub type AE14_R = crate::BitReader<bool>;
#[doc = "Field `AE14` writer - Acknowledge error 14"]
pub type AE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `AE15` reader - Acknowledge error 15"]
pub type AE15_R = crate::BitReader<bool>;
#[doc = "Field `AE15` writer - Acknowledge error 15"]
pub type AE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `PE0` reader - PHY error 0"]
pub type PE0_R = crate::BitReader<bool>;
#[doc = "Field `PE0` writer - PHY error 0"]
pub type PE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `PE1` reader - PHY error 1"]
pub type PE1_R = crate::BitReader<bool>;
#[doc = "Field `PE1` writer - PHY error 1"]
pub type PE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `PE2` reader - PHY error 2"]
pub type PE2_R = crate::BitReader<bool>;
#[doc = "Field `PE2` writer - PHY error 2"]
pub type PE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `PE3` reader - PHY error 3"]
pub type PE3_R = crate::BitReader<bool>;
#[doc = "Field `PE3` writer - PHY error 3"]
pub type PE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
#[doc = "Field `PE4` reader - PHY error 4"]
pub type PE4_R = crate::BitReader<bool>;
#[doc = "Field `PE4` writer - PHY error 4"]
pub type PE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Acknowledge error 0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge error 1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge error 2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge error 3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge error 4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Acknowledge error 5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge error 6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledge error 7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Acknowledge error 8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Acknowledge error 9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error 10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge error 11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Acknowledge error 12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Acknowledge error 13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Acknowledge error 14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Acknowledge error 15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PHY error 0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY error 1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY error 2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PHY error 3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PHY error 4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge error 0"]
    #[inline(always)]
    #[must_use]
    pub fn ae0(&mut self) -> AE0_W<0> {
        AE0_W::new(self)
    }
    #[doc = "Bit 1 - Acknowledge error 1"]
    #[inline(always)]
    #[must_use]
    pub fn ae1(&mut self) -> AE1_W<1> {
        AE1_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledge error 2"]
    #[inline(always)]
    #[must_use]
    pub fn ae2(&mut self) -> AE2_W<2> {
        AE2_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledge error 3"]
    #[inline(always)]
    #[must_use]
    pub fn ae3(&mut self) -> AE3_W<3> {
        AE3_W::new(self)
    }
    #[doc = "Bit 4 - Acknowledge error 4"]
    #[inline(always)]
    #[must_use]
    pub fn ae4(&mut self) -> AE4_W<4> {
        AE4_W::new(self)
    }
    #[doc = "Bit 5 - Acknowledge error 5"]
    #[inline(always)]
    #[must_use]
    pub fn ae5(&mut self) -> AE5_W<5> {
        AE5_W::new(self)
    }
    #[doc = "Bit 6 - Acknowledge error 6"]
    #[inline(always)]
    #[must_use]
    pub fn ae6(&mut self) -> AE6_W<6> {
        AE6_W::new(self)
    }
    #[doc = "Bit 7 - Acknowledge error 7"]
    #[inline(always)]
    #[must_use]
    pub fn ae7(&mut self) -> AE7_W<7> {
        AE7_W::new(self)
    }
    #[doc = "Bit 8 - Acknowledge error 8"]
    #[inline(always)]
    #[must_use]
    pub fn ae8(&mut self) -> AE8_W<8> {
        AE8_W::new(self)
    }
    #[doc = "Bit 9 - Acknowledge error 9"]
    #[inline(always)]
    #[must_use]
    pub fn ae9(&mut self) -> AE9_W<9> {
        AE9_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge error 10"]
    #[inline(always)]
    #[must_use]
    pub fn ae10(&mut self) -> AE10_W<10> {
        AE10_W::new(self)
    }
    #[doc = "Bit 11 - Acknowledge error 11"]
    #[inline(always)]
    #[must_use]
    pub fn ae11(&mut self) -> AE11_W<11> {
        AE11_W::new(self)
    }
    #[doc = "Bit 12 - Acknowledge error 12"]
    #[inline(always)]
    #[must_use]
    pub fn ae12(&mut self) -> AE12_W<12> {
        AE12_W::new(self)
    }
    #[doc = "Bit 13 - Acknowledge error 13"]
    #[inline(always)]
    #[must_use]
    pub fn ae13(&mut self) -> AE13_W<13> {
        AE13_W::new(self)
    }
    #[doc = "Bit 14 - Acknowledge error 14"]
    #[inline(always)]
    #[must_use]
    pub fn ae14(&mut self) -> AE14_W<14> {
        AE14_W::new(self)
    }
    #[doc = "Bit 15 - Acknowledge error 15"]
    #[inline(always)]
    #[must_use]
    pub fn ae15(&mut self) -> AE15_W<15> {
        AE15_W::new(self)
    }
    #[doc = "Bit 16 - PHY error 0"]
    #[inline(always)]
    #[must_use]
    pub fn pe0(&mut self) -> PE0_W<16> {
        PE0_W::new(self)
    }
    #[doc = "Bit 17 - PHY error 1"]
    #[inline(always)]
    #[must_use]
    pub fn pe1(&mut self) -> PE1_W<17> {
        PE1_W::new(self)
    }
    #[doc = "Bit 18 - PHY error 2"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<18> {
        PE2_W::new(self)
    }
    #[doc = "Bit 19 - PHY error 3"]
    #[inline(always)]
    #[must_use]
    pub fn pe3(&mut self) -> PE3_W<19> {
        PE3_W::new(self)
    }
    #[doc = "Bit 20 - PHY error 4"]
    #[inline(always)]
    #[must_use]
    pub fn pe4(&mut self) -> PE4_W<20> {
        PE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host interrupt and status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr0](index.html) module"]
pub struct ISR0_SPEC;
impl crate::RegisterSpec for ISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr0::R](R) reader structure"]
impl crate::Readable for ISR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr0::W](W) writer structure"]
impl crate::Writable for ISR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR0 to value 0"]
impl crate::Resettable for ISR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
