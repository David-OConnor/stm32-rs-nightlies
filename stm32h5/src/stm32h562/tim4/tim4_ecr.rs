#[doc = "Register `TIM4_ECR` reader"]
pub struct R(crate::R<TIM4_ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM4_ECR` writer"]
pub struct W(crate::W<TIM4_ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_ECR_SPEC>;
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
impl From<crate::W<TIM4_ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - Index enable This bit indicates if the Index event resets the counter."]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Index enable This bit indicates if the Index event resets the counter."]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM4_ECR_SPEC, bool, O>;
#[doc = "Field `IDIR` reader - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
pub type IDIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDIR` writer - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
pub type IDIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_ECR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IBLK` reader - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
pub type IBLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBLK` writer - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
pub type IBLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_ECR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIDX` reader - First index This bit indicates if the first index only is taken into account"]
pub type FIDX_R = crate::BitReader<bool>;
#[doc = "Field `FIDX` writer - First index This bit indicates if the first index only is taken into account"]
pub type FIDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM4_ECR_SPEC, bool, O>;
#[doc = "Field `IPOS` reader - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
pub type IPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPOS` writer - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
pub type IPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_ECR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PW` reader - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
pub type PW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PW` writer - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
pub type PW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_ECR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PWPRSC` reader - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
pub type PWPRSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWPRSC` writer - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
pub type PWPRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_ECR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Index enable This bit indicates if the Index event resets the counter."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - First index This bit indicates if the first index only is taken into account"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Index enable This bit indicates if the Index event resets the counter."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    #[doc = "Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
    #[inline(always)]
    #[must_use]
    pub fn idir(&mut self) -> IDIR_W<1> {
        IDIR_W::new(self)
    }
    #[doc = "Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
    #[inline(always)]
    #[must_use]
    pub fn iblk(&mut self) -> IBLK_W<3> {
        IBLK_W::new(self)
    }
    #[doc = "Bit 5 - First index This bit indicates if the first index only is taken into account"]
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FIDX_W<5> {
        FIDX_W::new(self)
    }
    #[doc = "Bits 6:7 - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
    #[inline(always)]
    #[must_use]
    pub fn ipos(&mut self) -> IPOS_W<6> {
        IPOS_W::new(self)
    }
    #[doc = "Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<16> {
        PW_W::new(self)
    }
    #[doc = "Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
    #[inline(always)]
    #[must_use]
    pub fn pwprsc(&mut self) -> PWPRSC_W<24> {
        PWPRSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM4 timer encoder control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ecr](index.html) module"]
pub struct TIM4_ECR_SPEC;
impl crate::RegisterSpec for TIM4_ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim4_ecr::R](R) reader structure"]
impl crate::Readable for TIM4_ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim4_ecr::W](W) writer structure"]
impl crate::Writable for TIM4_ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM4_ECR to value 0"]
impl crate::Resettable for TIM4_ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
